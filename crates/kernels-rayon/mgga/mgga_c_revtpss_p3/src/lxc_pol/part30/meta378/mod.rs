//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta378 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1425;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta378(t2516: f64, t5571: f64, t5566: f64, t72: f64, t757: f64, t1320: f64, t5567: f64, t5569: f64, t9395: f64, t9398: f64, t1353: f64, t1448: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t13612, t13615, t13620, t13622, t13623, t13624, t13625) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1425(t2516, t5571, t5566, t72, t757, t1320, t5567, t5569, t9395, t9398, t1353, t1448);
    (t13612, t13615, t13620, t13622, t13623, t13624, t13625)
}
