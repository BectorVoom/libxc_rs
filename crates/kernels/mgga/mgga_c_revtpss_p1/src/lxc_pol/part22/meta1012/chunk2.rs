//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3478/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3478<F: Float>(t1041: F, t1042: F, t1045: F, t1063: F, t11268: F, t16208: F, t19668: F, t19675: F, t247: F, t2862: F, t3127: F, t3182: F, t3188: F, t373: F, t42943: F, t4806: F, t6302: F, t6312: F, t63455: F, t65357: F, t65359: F, t65365: F, t65370: F, t65376: F, t65425: F, t65431: F, t65433: F) -> F {
    let t65438 = -F::cast_from(0.31758531939310916276e-4_f64) * t65357 - F::cast_from(0.10162730220579493208e-2_f64) * t65359 + F::cast_from(0.47637797908966374414e-3_f64) * t1063 * t247 * t3182 * t63455 - F::cast_from(0.23818898954483187207e-3_f64) * t3127 * t1042 * t4806 * t65365 + F::cast_from(0.63517063878621832552e-3_f64) * t1063 * t1042 * t16208 * t65370 - F::cast_from(0.19055119163586549765e-3_f64) * t65376 - F::cast_from(0.14291339372689912324e-3_f64) * t3127 * t1042 * t19675 * t2862 + F::cast_from(0.95275595817932748828e-3_f64) * t3188 * t19668 - F::cast_from(0.72409452821628889107e-2_f64) * t42943 * t6312 + F::cast_from(0.72409452821628889107e-2_f64) * t11268 * t6302 + F::cast_from(0.21437009059034868486e-3_f64) * t1041 * t1042 * t373 * t65425 * t1045 - F::cast_from(0.15244095330869239812e-2_f64) * t65431 + F::cast_from(0.47637797908966374414e-3_f64) * t1063 * t1042 * t4806 * t65433;
    t65438
}
