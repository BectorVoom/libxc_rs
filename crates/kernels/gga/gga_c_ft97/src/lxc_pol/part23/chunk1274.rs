//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1274/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1274<F: Float>(t123835: F, t123842: F, t123849: F, t108078: F, t108081: F, t108084: F, t108115: F, t108139: F, t108140: F, t123840: F, t123846: F, t123853: F, t123870: F, t123872: F, t123859: F, t123863: F, t123867: F, t123876: F, t123881: F, t123885: F, t123888: F, t123893: F, t123896: F, t123901: F) -> (F, F) {
    let t124509 = t123835 / 18.0;
    let t124511 = t123842 / 18.0;
    let t124514 = t123849 / 27.0;
    let t124515 = -t124509 - t123840 / 12.0 - t108078 - t108081 - t108084 - t108115 + t124511 - 6.0 * t123846 - t108139 + 8.0 / 27.0 * t108140 - t124514 - t123853;
    let t124519 = t123870 / 6.0;
    let t124520 = 2.0 / 9.0 * t123872;
    let t124528 = 3.0 / 2.0 * t123859 + t123863 / 3.0 + 2.0 / 9.0 * t123867 + t124519 + t124520 - 4.0 / 3.0 * t123876 + t123881 / 4.0 - 4.0 / 3.0 * t123885 + 4.0 / 9.0 * t123888 - 3.0 * t123893 + 8.0 / 3.0 * t123896 + t123901 / 4.0;
    (t124515, t124528)
}
