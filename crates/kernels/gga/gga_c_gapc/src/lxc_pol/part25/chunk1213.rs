//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1213/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1213<F: Float>(t33541: F, t33552: F, t33565: F, t36621: F, t36623: F, t36625: F, t36626: F, t36627: F, t36628: F, t36630: F, t36631: F, t33680: F, t33687: F, t33692: F, t36659: F, t36660: F, t36661: F, t36662: F, t36664: F, t36666: F, t36668: F, t36669: F) -> (F, F) {
    let t38747 = t36621 - 0.53808777420609085649e-7 * t33541 + t36623 - 0.89048050908546122982e-5 * t33552 - t36625 + t36626 - t36627 - t36628 + 0.12650553385416666667e-5 * t33565 + t36630 + t36631;
    let t38753 = -t36659 + t36660 - t36661 + t36662 - 0.2445773654513888889e-4 * t33680 + t36664 - 0.2445773654513888889e-4 * t33687 + t36666 + 0.56912804804009946682e-7 * t33692 - t36668 + t36669;
    (t38747, t38753)
}
