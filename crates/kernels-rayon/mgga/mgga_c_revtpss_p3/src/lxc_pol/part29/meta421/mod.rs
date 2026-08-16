//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta421 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1554;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1555;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta421(t1280: f64, t16750: f64, t3153: f64, t5284: f64, t5465: f64, t1287: f64, t1811: f64, t3588: f64, t13133: f64, t1774: f64, t1214: f64, t5245: f64, t3584: f64, t16641: f64, t16645: f64, t16647: f64, t16649: f64, t16651: f64, t16654: f64, t16657: f64, t16660: f64, t16664: f64, t16667: f64, t16671: f64, t16675: f64, t16679: f64, t16681: f64, t16684: f64, t16687: f64, t16690: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16751, t16756, t16757, t16763, t16768, t16771) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1554(t1280, t16750, t3153, t5284, t5465, t1287, t1811, t3588, t13133, t1774, t1214, t5245);
        let (t16772, t16775, t16776, t16781) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1555(t1280, t16771, t1774, t3584, t16641, t16645, t16647, t16649, t16651, t16654, t16657, t16660, t16664, t16667, t16671, t16675, t16679, t16681, t16684, t16687, t16690);
    (t16751, t16756, t16757, t16763, t16768, t16771, t16772, t16775, t16776, t16781)
}
