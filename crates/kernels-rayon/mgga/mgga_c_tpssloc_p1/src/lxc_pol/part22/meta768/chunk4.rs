//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2606/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2606(t14725: f64, t17635: f64, t18329: f64, t4889: f64, t18324: f64, t1174: f64, t135: f64, t22136: f64, t18346: f64, t18580: f64, t19019: f64, t3440: f64, t45128: f64, t5024: f64, t52873: f64, t52893: f64, t66001: f64, t66015: f64, t66024: f64, t66027: f64, t71138: f64) -> (f64, f64) {
    let t72688 = t14725 * t17635;
    let t72703 = t4889 * t18329;
    let t72705 = t4889 * t18324;
    let t72708 = t1174 * t135 * t22136;
    let t72712 = -5.0_f64 / 1728.0_f64 * t52893 * t45128 * t72688 - t66001 / 144.0_f64 - t4889 * t19019 / 27.0_f64 - 2.0_f64 / 9.0_f64 * t4889 * t18580 + t1174 * t3440 * t71138 / 216.0_f64 + t66015 / 216.0_f64 + t52873 + 5.0_f64 / 3456.0_f64 * t66024 + 5.0_f64 / 1152.0_f64 * t66027 + t72703 / 108.0_f64 + t72705 / 54.0_f64 + t72708 / 108.0_f64 - 5.0_f64 / 144.0_f64 * t5024 * t18346;
    (t72688, t72712)
}
