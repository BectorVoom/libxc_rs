//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2076/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2076<F: Float>(t26050: F, t26072: F, t213: F, t26034: F, t25899: F, t94664: F, t94404: F, t2453: F, t25949: F, t25946: F, t25939: F, t40270: F) -> (F, F, F, F, F, F) {
    let t94904 = t26072 * t26050;
    let t94906 = t213 * t26034;
    let t94909 = t25899 * t94664;
    let t94911 = t25899 * t94404;
    let t94913 = t2453 * t25949;
    let t94914 = t94913 * t25946;
    let t94917 = F::cast_from(0.96373646535613327356e-3_f64) * t40270 * t25939;
    (t94904, t94906, t94909, t94911, t94914, t94917)
}
