//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1238/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1238<F: Float>(t24537: F, t25116: F, t809: F, t10528: F, t10625: F, t10662: F, t10668: F, t10705: F, t2289: F, t2322: F, t24530: F, t260: F, t28932: F, t28937: F, t28949: F, t28962: F, t4193: F, t4215: F, t6666: F, t6737: F, t6759: F, t839: F, t848: F, t856: F, t8600: F, t8776: F) -> (F, F) {
    let t28967 = 0.2069040516770936012e4 * t24537 * t25116 * t809;
    let t28968 = -0.70178683471615754484e1 * t2322 * t10528 + 0.46785788981077169656e1 * t2322 * t10662 + 0.20779030926817756511e3 * t2322 * t10668 - 0.11696447245269292414e1 * t2322 * t10705 - 0.17315859105681463759e2 * t6759 * t4215 - 0.5848223622634646207e0 * t856 * t839 * t28932 * t848 + 0.23392894490538584828e1 * t856 * t2289 * t28937 * t848 - 0.17315859105681463759e2 * t856 * t10625 * t6737 - 0.10254018858216406658e4 * t856 * t6666 * t4193 * t8600 - 0.41016075432865626631e4 * t24530 * t8776 * t28949 + 0.19751673498613801407e-1 * t260 * t28962 + t28967;
    (t28967, t28968)
}
