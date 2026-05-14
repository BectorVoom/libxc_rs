//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 989/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk989<F: Float>(t88252: F, t9570: F, t2404: F, t92: F, t88184: F, t2347: F, t88239: F, t88153: F, t9577: F, t683: F, t88606: F, t2360: F, t88149: F, t41446: F, t9568: F, t88612: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t88735 = t9570 * t88252;
    let t88737 = t92 * t2404 * t88735;
    let t88740 = t92 * t2404 * t88184;
    let t88742 = t2347 * t88239;
    let t88744 = t92 * t2404 * t88742;
    let t88747 = t92 * t2404 * t88153;
    let t88749 = t9577 * t88252;
    let t88751 = t92 * t683 * t88749;
    let t88754 = t92 * t683 * t88606;
    let t88756 = t2360 * t88239;
    let t88758 = t92 * t683 * t88756;
    let t88761 = t92 * t683 * t88149;
    let t88764 = t41446 * t88252;
    let t88766 = t92 * t9568 * t88764;
    let t88769 = t92 * t9568 * t88612;
    (t88735, t88737, t88740, t88742, t88744, t88747, t88749, t88751, t88754, t88756, t88758, t88761, t88764, t88766, t88769)
}
