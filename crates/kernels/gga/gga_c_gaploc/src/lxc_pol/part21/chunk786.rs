//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 786/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk786<F: Float>(t2089: F, t911: F, t7419: F, t7427: F, t2604: F, t6135: F, t1835: F, t733: F, t2365: F, t2022: F, t701: F, t7291: F) -> (F, F, F, F) {
    let t7428 = t911 * t2089;
    let t7429 = t7428 * t7419;
    let t7430 = t7427 * t7429;
    let t7432 = t6135 * t2604;
    let t7434 = t733 * t1835;
    let t7435 = t2365 * t7434;
    let t7436 = t2022 * t7435;
    let t7438 = t7291 * t701;
    (t7430, t7432, t7436, t7438)
}
