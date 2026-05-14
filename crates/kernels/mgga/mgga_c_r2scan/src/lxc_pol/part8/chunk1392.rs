//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1392/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1392<F: Float>(t2823: F, t29002: F, t19720: F, t22636: F, t22647: F, t32978: F, t32979: F, t32980: F, t32982: F, t32984: F, t32985: F, t32986: F, t32987: F, t19611: F, t19614: F, t19620: F, t19624: F, t19628: F, t19646: F, t19649: F, t19687: F, t19728: F, t23951: F, t32990: F) -> (F, F) {
    let t33777 = t2823 * t29002;
    let t33780 = t32978 + t32979 - 0.2025780996e0 * t33777 - t32980 + 0.285764e-1 * t22636 + t22647 - t32982 - t32984 + t32985 + t32986 + t32987 - t19720;
    let t33783 = -t19611 - t19614 + t19620 - t19624 + t19628 + t19646 + t19649 + t19728 - t19687 - t23951 + t32990;
    (t33780, t33783)
}
