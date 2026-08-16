//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2591/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2591<F: Float>(t15438: F, t19095: F, t19083: F, t4993: F, t18392: F, t5024: F, t1226: F, t22115: F, t11692: F, t1174: F, t1177: F, t1232: F, t15700: F, t15740: F, t1735: F, t18221: F, t18397: F, t18401: F, t19010: F, t19106: F, t3440: F, t3577: F, t3578: F, t4889: F, t52766: F, t53298: F, t5392: F, t65528: F, t71172: F, t71193: F) -> F {
    let t72248 = t15438 * t19095;
    let t72251 = t19083 * t4993;
    let t72253 = t5024 * t18392;
    let t72255 = t22115 * t1226;
    let t72268 = t52766 * t18397 / F::cast_from(768.0_f64) - t15740 * t18401 / F::cast_from(384.0_f64) + t11692 * t3578 * t15700 * t53298 * t5392 / F::cast_from(768.0_f64) - t3577 * t3578 * t1735 * t18221 / F::cast_from(256.0_f64) - t72248 / F::cast_from(1536.0_f64) - t65528 / F::cast_from(4608.0_f64) + t72251 / F::cast_from(216.0_f64) + t72253 / F::cast_from(216.0_f64) - t72255 * t1232 / F::cast_from(4608.0_f64) + t4889 * t19010 / F::cast_from(18.0_f64) - t1174 * t1177 * t71172 / F::cast_from(12.0_f64) + t1174 * t3440 * t71193 / F::cast_from(12.0_f64) + F::cast_from(7.0_f64) / F::cast_from(81.0_f64) * t4889 * t19106;
    t72268
}
