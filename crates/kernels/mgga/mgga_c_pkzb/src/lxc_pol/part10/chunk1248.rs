//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1248/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1248<F: Float>(t496: F, t8775: F, t16552: F, t16554: F, t16571: F, t16580: F, t16582: F, t1542: F, t3426: F, t1508: F, t8770: F, t114: F, t557: F, t8748: F, t16539: F, t16544: F, t16548: F, t16550: F, t16563: F, t16569: F, t16575: F, t16578: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t24527 = t496 * t8775;
    let t24528 = 8.0 * t24527;
    let t24529 = 0.11696447245269292414e1 * t16552;
    let t24530 = 0.11696447245269292414e1 * t16554;
    let t24531 = 240.0 * t16571;
    let t24532 = 0.10843581300301739842e-1 * t16580;
    let t24533 = 0.96319466275353142156e0 * t16582;
    let t24534 = t1542 * t3426;
    let t24535 = 20.0 * t24534;
    let t24536 = t8770 * t1508;
    let t24537 = 0.17315859105681463759e2 * t24536;
    let t24539 = t8748 * t114 * t557;
    let t24540 = 0.11696447245269292414e1 * t24539;
    let t24541 = -t16539 - t16544 + t16548 + t16550 + t24528 + t24529 - t24530 - t16563 + t16569 - t24531 + t16575 + t16578 + t24532 + t24533 + t24535 - t24537 - t24540;
    (t24528, t24529, t24530, t24531, t24532, t24533, t24535, t24537, t24540, t24541)
}
