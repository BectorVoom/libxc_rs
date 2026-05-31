//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 637/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk637<F: Float>(t2630: F, t3869: F, t1337: F, t2619: F, t514: F, t517: F, t1359: F, t2435: F, t555: F, t785: F, t1358: F, t2439: F) -> (F, F, F, F, F, F, F, F) {
    let t3871 = F::cast_from(0.10843581300301739842e-1_f64) * t3869 * t2630;
    let t3873 = F::cast_from(0.24415263074675393405e-3_f64) * t1337 * t2619;
    let t3874 = F::cast_from(1.0_f64) / t514;
    let t3881 = F::cast_from(1.0_f64) / t517;
    let t3894 = F::cast_from(0.73171657588172351096e-2_f64) * t2435 * t1359;
    let t3895 = t785 * t555;
    let t3896 = t3895 * t1358;
    let t3898 = F::cast_from(0.65049603595885220126e-3_f64) * t2439 * t3896;
    (t3871, t3873, t3874, t3881, t3894, t3895, t3896, t3898)
}
