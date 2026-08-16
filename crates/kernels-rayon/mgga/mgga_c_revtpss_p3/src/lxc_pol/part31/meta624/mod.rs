//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta624 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2075;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta624(t93190: f64, t99211: f64, t25374: f64, t98848: f64, t25378: f64, t99403: f64, t25375: f64, t99161: f64, t1580: f64, t25338: f64, t689: f64, t25365: f64, t27279: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t99460, t99463, t99465, t99466, t99468, t99472, t99475, t99480) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2075(t93190, t99211, t25374, t98848, t25378, t99403, t25375, t99161, t1580, t25338, t689, t25365, t27279);
    (t99460, t99463, t99465, t99466, t99468, t99472, t99475, t99480)
}
