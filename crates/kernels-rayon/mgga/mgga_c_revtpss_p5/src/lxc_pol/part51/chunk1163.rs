//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1163/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1163(t11921: f64, t247: f64, t31913: f64, t33749: f64, t32010: f64, t4845: f64, t1042: f64, t120251: f64, t120361: f64, t120397: f64, t120403: f64, t120406: f64, t120448: f64, t120558: f64, t120625: f64, t126660: f64, t1669: f64, t19634: f64, t25698: f64, t27652: f64, t31891: f64, t31892: f64, t33751: f64, t33827: f64, t4757: f64, t4788: f64, t4976: f64, t7135: f64, t7145: f64, t7810: f64, t99970: f64) -> f64 {
    let t126689 = t31913 * t247 * t11921 * t33749;
    let t126702 = t32010 * t4845;
    let t126708 = 0.34271842599061411569e1_f64 * t120625 * t126660 * t27652 - 0.34271842599061411569e1_f64 * t120558 * t126660 * t4976 + 0.11423947533020470523e1_f64 * t31891 * t31892 * t7135 * t7810 - 0.30116764542379164799e-2_f64 * t120403 * t33751 + 0.37645955677973955999e-3_f64 * t126689 + 0.34694512752820797848e1_f64 * t120448 * t7145 * t99970 - 0.24791552806034007214e-3_f64 * t120397 * t4788 - 0.52041769129231196772e1_f64 * t25698 * t120361 * t7145 * t4757 + 0.3718732920905101082e-3_f64 * t120406 * t33827 + 0.3718732920905101082e-3_f64 * t126702 - 0.11156198762715303246e-2_f64 * t120251 * t1042 * t1669 * t19634;
    t126708
}
