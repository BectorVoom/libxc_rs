//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1328/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1328(t24537: f64, t25116: f64, t809: f64, t10528: f64, t10625: f64, t10662: f64, t10668: f64, t10705: f64, t2289: f64, t2322: f64, t24530: f64, t260: f64, t28932: f64, t28937: f64, t28949: f64, t28962: f64, t4193: f64, t4215: f64, t6666: f64, t6737: f64, t6759: f64, t839: f64, t848: f64, t856: f64, t8600: f64, t8776: f64) -> (f64, f64) {
    let t28967 = 0.2069040516770936012e4_f64 * t24537 * t25116 * t809;
    let t28968 = -0.70178683471615754484e1_f64 * t2322 * t10528 + 0.46785788981077169656e1_f64 * t2322 * t10662 + 0.20779030926817756511e3_f64 * t2322 * t10668 - 0.11696447245269292414e1_f64 * t2322 * t10705 - 0.17315859105681463759e2_f64 * t6759 * t4215 - 0.5848223622634646207e0_f64 * t856 * t839 * t28932 * t848 + 0.23392894490538584828e1_f64 * t856 * t2289 * t28937 * t848 - 0.17315859105681463759e2_f64 * t856 * t10625 * t6737 - 0.10254018858216406658e4_f64 * t856 * t6666 * t4193 * t8600 - 0.41016075432865626631e4_f64 * t24530 * t8776 * t28949 + 0.19751673498613801407e-1_f64 * t260 * t28962 + t28967;
    (t28967, t28968)
}
