//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2373/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2373<F: Float>(t10852: F, t40336: F, t10858: F, t10863: F, t10868: F, t820: F, t843: F, t10874: F, t2482: F, t27: F, t10872: F, t221: F, t2485: F) -> (F, F, F, F) {
    let t40337 = t40336 * t10852;
    let t40345 = t10858 * t10863;
    let t40348 = t820 * t10868 * t843;
    let t40349 = t40348 * t10874;
    let t40352 = t2482 * t10868 * t27;
    let t40355 = t40352 * t2485 * t221 * t10872;
    (t40337, t40345, t40349, t40355)
}
