//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2600/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2600<F: Float>(t1227: F, t13969: F, t22257: F, t21769: F, t248: F, t3521: F, t22157: F, t3577: F, t45124: F, t11668: F, t11709: F, t1216: F, t15659: F, t18303: F, t18307: F, t18943: F, t18959: F, t21776: F, t22246: F, t22271: F, t3506: F, t3536: F, t3578: F, t4582: F, t5005: F, t5012: F, t5019: F, t52810: F, t53238: F, t53472: F, t5971: F, t6227: F, t66533: F) -> F {
    let t72495 = t1227 * t13969 * t22257;
    let t72501 = t1227 * t248 * t3521 * t21769;
    let t72512 = t3577 * t45124 * t22157;
    let t72522 = t11709 * t22271 / F::cast_from(512.0_f64) + t3506 * t4582 * t66533 * t15659 / F::cast_from(512.0_f64) - t5005 * t18959 / F::cast_from(768.0_f64) - t72495 / F::cast_from(1152.0_f64) + t3536 * t22246 / F::cast_from(3072.0_f64) - t72501 / F::cast_from(1152.0_f64) + F::cast_from(3.0_f64) / F::cast_from(512.0_f64) * t53238 * t18303 - F::cast_from(3.0_f64) / F::cast_from(512.0_f64) * t53472 * t18307 - t5019 * t18943 / F::cast_from(192.0_f64) - t52810 * t6227 / F::cast_from(96.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t72512 - t3577 * t3578 * t21776 * t1216 / F::cast_from(4608.0_f64) + F::cast_from(5.0_f64) / F::cast_from(4608.0_f64) * t3577 * t11668 * t5012 * t5971;
    t72522
}
