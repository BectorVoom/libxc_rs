//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1100/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1100<F: Float>(t1358: F, t3895: F, t2439: F, t1419: F, t212: F) -> (F, F, F) {
    let t3896 = t3895 * t1358;
    let t3898 = F::cast_from(0.65049603595885220126e-3_f64) * t2439 * t3896;
    let t3899 = t212 * t1419;
    (t3896, t3898, t3899)
}
