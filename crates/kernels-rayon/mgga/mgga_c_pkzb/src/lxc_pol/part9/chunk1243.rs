//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1243/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1243(t54: f64, t7699: f64, t2899: f64, t7769: f64, t2922: f64, t7702: f64, t17953: f64, t17957: f64, t18024: f64, t18026: f64, t18028: f64, t18033: f64, t18036: f64, t2009: f64, t2027: f64, t2031: f64, t5956: f64, t5978: f64, t655: f64, t7700: f64, t7703: f64, t7725: f64, t7736: f64) -> f64 {
    let t21787 = t54 * t7699;
    let t21789 = t2899 * t21787 * t7769;
    let t21794 = t2922 * t21787 * t7702;
    let t21803 = -0.25724410870841842183e-2_f64 * t2899 * t7700 * t2031 * t2009 * t655 - 0.25724410870841842183e-2_f64 * t2899 * t7700 * t2031 * t5978 - 0.77173232612525526551e-2_f64 * t7736 * t7700 * t5956 * t2027 * t655 - 0.34299214494455789577e-2_f64 * t21789 - 0.13719685797782315831e-1_f64 * t7725 * t7703 + 0.17149607247227894789e-2_f64 * t21794 + 0.15244095330869239812e-2_f64 * t17953 + 0.19055119163586549765e-3_f64 * t17957 - 0.22866142996303859718e-2_f64 * t18024 - 0.45732285992607719436e-2_f64 * t18026 + 0.22866142996303859718e-2_f64 * t18028 - 0.14291339372689912324e-3_f64 * t18033 - 0.28582678745379824648e-3_f64 * t18036;
    t21803
}
