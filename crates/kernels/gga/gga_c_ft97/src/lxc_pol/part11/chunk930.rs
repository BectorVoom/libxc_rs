//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 930/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk930<F: Float>(t238: F, t41588: F, t41635: F, t41681: F, t41791: F, t27: F, t676: F, t89: F, t1636: F, t2460: F, t375: F, t9693: F, t2999: F, t714: F, t1882: F, t9758: F, t9741: F) -> (F, F, F, F, F, F, F, F, F) {
    let t239 = 0.1e-59 < t238;
    let t41794 = piecewise3(t239, t41588 + t41635 + t41681 + t41791, 0.0);
    let t41797 = t89 * t27 * t676 * t41794;
    let t41800 = t89 * t1636 * t2460;
    let t41801 = 4.0 / 9.0 * t41800;
    let t41803 = t89 * t375 * t9693;
    let t41806 = t89 * t2999 * t714;
    let t41807 = 56.0 / 81.0 * t41806;
    let t41808 = t1882 * t9758;
    let t41810 = t1882 * t9741;
    (t41794, t41797, t41800, t41801, t41803, t41806, t41807, t41808, t41810)
}
