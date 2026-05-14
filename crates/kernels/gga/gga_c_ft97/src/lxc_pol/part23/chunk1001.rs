//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1001/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1001<F: Float>(t24260: F, t5025: F, t5049: F, t6027: F, t1701: F, t5005: F, t237: F, t39: F, t1100: F, t1127: F, t27494: F, t6979: F, t213: F, t231: F, t6819: F, t27506: F, t6832: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t30794 = t24260 * t5025;
    let t30807 = t6027 * t5049;
    let t30812 = t1701 * t6027 * t5005;
    let t30815 = t237 * t39;
    let t30816 = t1100 * t30815;
    let t30825 = t6027 * t5025;
    let t30829 = t27494 * t1127;
    let t30833 = t6979 * t1127;
    let t30838 = t213 * t1127;
    let t30839 = t231 * t30838;
    let t30840 = t6819 * t30839;
    let t30843 = t27506 * t6832;
    (t30794, t30807, t30812, t30816, t30825, t30829, t30833, t30838, t30839, t30840, t30843)
}
