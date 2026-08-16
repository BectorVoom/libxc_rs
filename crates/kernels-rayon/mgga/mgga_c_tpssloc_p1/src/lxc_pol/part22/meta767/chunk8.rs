//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2600/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2600(t1227: f64, t13969: f64, t22257: f64, t21769: f64, t248: f64, t3521: f64, t22157: f64, t3577: f64, t45124: f64, t11668: f64, t11709: f64, t1216: f64, t15659: f64, t18303: f64, t18307: f64, t18943: f64, t18959: f64, t21776: f64, t22246: f64, t22271: f64, t3506: f64, t3536: f64, t3578: f64, t4582: f64, t5005: f64, t5012: f64, t5019: f64, t52810: f64, t53238: f64, t53472: f64, t5971: f64, t6227: f64, t66533: f64) -> f64 {
    let t72495 = t1227 * t13969 * t22257;
    let t72501 = t1227 * t248 * t3521 * t21769;
    let t72512 = t3577 * t45124 * t22157;
    let t72522 = t11709 * t22271 / 512.0_f64 + t3506 * t4582 * t66533 * t15659 / 512.0_f64 - t5005 * t18959 / 768.0_f64 - t72495 / 1152.0_f64 + t3536 * t22246 / 3072.0_f64 - t72501 / 1152.0_f64 + 3.0_f64 / 512.0_f64 * t53238 * t18303 - 3.0_f64 / 512.0_f64 * t53472 * t18307 - t5019 * t18943 / 192.0_f64 - t52810 * t6227 / 96.0_f64 + 5.0_f64 / 6912.0_f64 * t72512 - t3577 * t3578 * t21776 * t1216 / 4608.0_f64 + 5.0_f64 / 4608.0_f64 * t3577 * t11668 * t5012 * t5971;
    t72522
}
