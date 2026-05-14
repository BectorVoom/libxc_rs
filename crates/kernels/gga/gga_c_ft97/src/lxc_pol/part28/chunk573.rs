//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 573/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk573<F: Float>(t25802: F, t73: F, t22632: F, t5598: F, t6445: F, t22652: F, t938: F, t6427: F, t7839: F, t25653: F, t5540: F, t22743: F, t25774: F, t25779: F, t401: F, t72: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t25803 = t73 * t25802;
    let t25813 = t5598 * t22632 * t6445;
    let t25816 = t22652 * t938;
    let t25820 = t6427 * t7839;
    let t25826 = t5540 * t25653;
    let t25829 = t5540 * t25802;
    let t25832 = t22743 * t25774;
    let t25835 = t5540 * t25779;
    let t25838 = t938 * t401;
    let t25839 = t72 * t25838;
    (t25803, t25813, t25816, t25820, t25826, t25829, t25832, t25835, t25838, t25839)
}
