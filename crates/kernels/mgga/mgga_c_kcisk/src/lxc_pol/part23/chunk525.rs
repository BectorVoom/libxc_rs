//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 525/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk525<F: Float>(t1433: F, t3619: F, t457: F, t1216: F, t3571: F, t3573: F, t3577: F, t3581: F, t3585: F, t321: F, t1167: F, t1171: F, t1192: F, t1170: F, t317: F, t305: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3620 = t1433 * t3619;
    let t3621 = t457 * t3620;
    let t3624 = t1216 * t1216;
    let t3626 = 0.23744444444444444444e-1 * t3571;
    let t3631 = t3626 + 0.11872222222222222222e-1 * t3573 - 0.11872222222222222222e-1 * t3577 + 0.35616666666666666666e-1 * t3581 - 0.17808333333333333333e-1 * t3585;
    let t3633 = 0.62182e-1 * t3631 * t321;
    let t3634 = t1167 * t1171;
    let t3636 = 2.0 * t3634 * t1192;
    let t3637 = t1170 * t317;
    let t3638 = 1.0 / t3637;
    let t3639 = t305 * t3638;
    (t3620, t3621, t3624, t3626, t3631, t3633, t3634, t3636, t3638, t3639)
}
