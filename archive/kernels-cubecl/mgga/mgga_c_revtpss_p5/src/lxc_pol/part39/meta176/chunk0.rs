//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 770/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk770<F: Float>(t1359: F, t2435: F, t555: F, t785: F, t1358: F, t2439: F, t1419: F, t212: F, t689: F, t1357: F, t1445: F, t2453: F, t556: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3894 = F::cast_from(0.73171657588172351096e-2_f64) * t2435 * t1359;
    let t3895 = t785 * t555;
    let t3896 = t3895 * t1358;
    let t3898 = F::cast_from(0.65049603595885220126e-3_f64) * t2439 * t3896;
    let t3899 = t212 * t1419;
    let t3900 = t3899 * t1358;
    let t3901 = t689 * t3900;
    let t3903 = t1357 * t1445;
    let t3904 = t689 * t3903;
    let t3906 = t2453 * t556;
    (t3894, t3895, t3896, t3898, t3899, t3900, t3901, t3903, t3904, t3906)
}
