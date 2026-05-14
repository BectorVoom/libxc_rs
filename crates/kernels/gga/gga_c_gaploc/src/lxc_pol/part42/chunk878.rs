//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 878/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk878<F: Float>(t224: F, t50308: F, t50312: F, t50478: F, t50800: F, t14443: F, t45164: F, t45973: F, t45974: F, t45992: F, t46006: F, t46023: F, t46025: F, t46835: F, t49977: F, t49980: F, t49983: F, t50475: F, t50781: F, t50789: F, t50791: F, t50796: F, t50799: F, t617: F) -> (F,) {
    let t50803 = t224 * (t50308 + t50312 + t50478 + t50800);
    let t50805 = t14443 * t617 - t45164 + t45973 - t45974 + t45992 + t46006 + t46023 + t46025 + t46835 - t49977 + t49980 - t49983 + t50475 - t50781 + t50789 - t50791 - t50796 + t50799 + t50803;
    (t50805,)
}
