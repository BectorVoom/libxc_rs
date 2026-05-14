//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 872/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk872<F: Float>(t13887: F, t22928: F, t22929: F, t22930: F, t22931: F, t9524: F, t9542: F, t9588: F, t9598: F, t9854: F, t9857: F, t9865: F, t9868: F, t225: F, t22917: F, t22923: F, t22927: F) -> (F, F) {
    let t22932 = 0.73245789224026180216e-3 * t13887;
    let t22933 = -t9588 - t9524 + t9598 - t22928 + t22929 + t22930 + t22931 + t9542 - t9854 - t9857 + t9865 + t9868 + t22932;
    let t22936 = (t22917 + t22923 + t22927 + t22933) * t225;
    (t22932, t22936)
}
