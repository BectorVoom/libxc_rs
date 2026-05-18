//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1431/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1431<F: Float>(t33507: F, t36609: F, t36610: F, t36611: F, t36612: F, t36613: F, t36615: F, t36616: F, t36617: F, t36618: F, t36619: F, t33541: F, t33552: F, t33565: F, t36621: F, t36623: F, t36625: F, t36626: F, t36627: F, t36628: F, t36630: F, t36631: F) -> (F, F) {
    let t38743 = -t36609 - t36610 + t36611 - t36612 + t36613 + F::new(0.67632724766374884054e-5) * t33507 - t36615 - t36616 + t36617 + t36618 - t36619;
    let t38747 = t36621 - F::new(0.53808777420609085649e-7) * t33541 + t36623 - F::new(0.89048050908546122982e-5) * t33552 - t36625 + t36626 - t36627 - t36628 + F::new(0.12650553385416666667e-5) * t33565 + t36630 + t36631;
    (t38743, t38747)
}
