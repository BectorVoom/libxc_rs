//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta573 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1982;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta573(t2246: f64, t5812: f64, t10871: f64, t5977: f64, t18493: f64, t221: f64, t18498: f64, t6016: f64, t836: f64, t18435: f64, t6022: f64, t23160: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t60673, t61639, t61725, t61749, t61756, t62403, t62589, t62593, t62604) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1982(t2246, t5812, t10871, t5977, t18493, t221, t18498, t6016, t836, t18435, t6022, t23160);
    (t60673, t61639, t61725, t61749, t61756, t62403, t62589, t62593, t62604)
}
