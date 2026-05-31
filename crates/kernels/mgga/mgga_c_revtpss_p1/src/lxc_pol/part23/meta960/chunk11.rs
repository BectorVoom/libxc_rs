//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3244/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3244<F: Float>(t5876: F, t670: F, t13426: F, t1519: F, t18227: F, t18242: F, t18245: F, t21882: F, t21891: F, t22578: F, t2322: F, t27126: F, t4248: F, t4254: F, t4257: F, t4293: F, t5517: F, t5887: F, t5920: F, t5921: F, t651: F, t75439: F, t7732: F) -> (F, F) {
    let t85360 = t5876 * t670;
    let t85373 = -F::cast_from(6.0_f64) * t5517 * t5920 * t651 - F::cast_from(12.0_f64) * t13426 * t5887 - F::cast_from(6.0_f64) * t1519 * t75439 - F::cast_from(6.0_f64) * t1519 * t85360 - F::cast_from(12.0_f64) * t18227 * t5887 - F::cast_from(6.0_f64) * t18242 * t7732 - F::cast_from(6.0_f64) * t18245 * t4257 - F::cast_from(6.0_f64) * t18245 * t4293 - F::cast_from(6.0_f64) * t21882 * t7732 - F::cast_from(12.0_f64) * t21891 * t4248 - F::cast_from(6.0_f64) * t22578 * t2322 - F::cast_from(6.0_f64) * t22578 * t4254 - F::cast_from(6.0_f64) * t27126 * t5921;
    (t85360, t85373)
}
