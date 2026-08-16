//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1243/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1243<F: Float>(t54: F, t7699: F, t2899: F, t7769: F, t2922: F, t7702: F, t17953: F, t17957: F, t18024: F, t18026: F, t18028: F, t18033: F, t18036: F, t2009: F, t2027: F, t2031: F, t5956: F, t5978: F, t655: F, t7700: F, t7703: F, t7725: F, t7736: F) -> F {
    let t21787 = t54 * t7699;
    let t21789 = t2899 * t21787 * t7769;
    let t21794 = t2922 * t21787 * t7702;
    let t21803 = -F::cast_from(0.25724410870841842183e-2_f64) * t2899 * t7700 * t2031 * t2009 * t655 - F::cast_from(0.25724410870841842183e-2_f64) * t2899 * t7700 * t2031 * t5978 - F::cast_from(0.77173232612525526551e-2_f64) * t7736 * t7700 * t5956 * t2027 * t655 - F::cast_from(0.34299214494455789577e-2_f64) * t21789 - F::cast_from(0.13719685797782315831e-1_f64) * t7725 * t7703 + F::cast_from(0.17149607247227894789e-2_f64) * t21794 + F::cast_from(0.15244095330869239812e-2_f64) * t17953 + F::cast_from(0.19055119163586549765e-3_f64) * t17957 - F::cast_from(0.22866142996303859718e-2_f64) * t18024 - F::cast_from(0.45732285992607719436e-2_f64) * t18026 + F::cast_from(0.22866142996303859718e-2_f64) * t18028 - F::cast_from(0.14291339372689912324e-3_f64) * t18033 - F::cast_from(0.28582678745379824648e-3_f64) * t18036;
    t21803
}
