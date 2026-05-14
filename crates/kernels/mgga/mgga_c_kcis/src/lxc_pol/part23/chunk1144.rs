//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1144/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1144<F: Float>(t7968: F, t99056: F, t18210: F, t28707: F, t27595: F, t27607: F, t28741: F, t18159: F, t27583: F, t27598: F, t27665: F, t28727: F, t28853: F, t98052: F, t98055: F, t98069: F, t99043: F, t99046: F, t99052: F) -> (F, F) {
    let t99058 = 0.30918233506944444444e-4 * t7968 * t99056;
    let t99059 = t18210 * t28707;
    let t99060 = t27595 * t99059;
    let t99065 = 0.7722800925925925926e-4 * t27607 * t28741;
    let t99066 = -0.23168402777777777778e-3 * t27583 * t99043 - 0.46336805555555555556e-3 * t27583 * t99046 * t18159 - 0.30952962962962962962e-2 * t98052 - 0.51588271604938271603e-2 * t98055 - t99052 + 0.37101880208333333334e-3 * t28853 * t27598 + 0.34822083333333333332e-2 * t98069 - t99058 - 0.61890573922526041667e-5 * t99060 - 0.61782407407407407408e-3 * t28727 * t27665 - t99065;
    (t99059, t99066)
}
