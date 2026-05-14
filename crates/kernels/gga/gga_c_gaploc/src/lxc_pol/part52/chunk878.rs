//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 878/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk878<F: Float>(t45164: F, t45973: F, t45974: F, t45992: F, t46006: F, t46023: F, t46025: F, t46835: F, t49974: F, t49977: F, t49980: F, t49983: F, t50475: F, t50781: F, t50789: F, t50791: F, t50796: F, t50799: F, t50803: F) -> (F,) {
    let t51229 = -t49974 - t45164 - t49977 + t45973 + t49980 - t49983 - t45974 + t45992 + t50803 + t50475 + t46006 - t50781 + t50789 + t46023 - t50791 + t46025 - t50796 + t50799 + t46835;
    (t51229,)
}
