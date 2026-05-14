//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 584/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk584<F: Float>(t1060: F, t3336: F, t783: F, t1779: F, t9: F, t2096: F, t2105: F, t265: F) -> (F, F, F, F) {
    let t3338 = t783 * t3336 * t1060;
    let t3341 = 1.0 / t9 / t1779;
    let t3342 = t2096 * t3341;
    let t3344 = t3342 * t265 * t2105;
    (t3338, t3341, t3342, t3344)
}
