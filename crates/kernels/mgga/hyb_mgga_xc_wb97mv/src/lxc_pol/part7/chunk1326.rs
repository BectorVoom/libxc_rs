//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1326/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1326<F: Float>(t23559: F, t23561: F, t23563: F, t23567: F, t23570: F, t23574: F, t23577: F, t23578: F, t23582: F, t27761: F, t27763: F, t27766: F, t27768: F, t27772: F, t27774: F, t27776: F, t27781: F, t27783: F) -> (F,) {
    let t32434 = -0.11696447245269292414e1 * t23559 - 0.17315859105681463759e2 * t23561 - 0.11696447245269292414e1 * t27761 - 0.70178683471615754484e1 * t27763 - 0.11696447245269292414e1 * t27766 + 0.46785788981077169656e1 * t27768 + 0.11696447245269292414e1 * t23563 + t23567 - t23570 - t23574 - t23577 - 0.65061487801810439052e-1 * t23578 + t23582 + 0.32530743900905219526e-1 * t27772 - 0.43374325201206959368e-1 * t27774 - 0.65061487801810439052e-1 * t27776 + 0.43374325201206959368e-1 * t27781 + 96.0 * t27783;
    (t32434,)
}
