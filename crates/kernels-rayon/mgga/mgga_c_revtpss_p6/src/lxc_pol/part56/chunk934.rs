//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 934/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk934(t1444: f64, t32211: f64, t5673: f64, t32206: f64, t1032: f64, t8578: f64, t1426: f64, t786: f64, t545: f64, t72: f64, t686: f64, t7063: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32213 = t5673 * t32211 * t1444;
    let t32214 = t32206 * t32213;
    let t32216 = t8578 * t1032;
    let t32217 = t32216 * t1426;
    let t32218 = t786 * t32217;
    let t32219 = t545 * t72;
    let t32220 = t32219 * t686;
    let t32222 = 0.14456046980341999104e-1_f64 * t32218 * t32220;
    let t32223 = t7063 * t32217;
    (t32213, t32214, t32216, t32217, t32218, t32219, t32220, t32222, t32223)
}
