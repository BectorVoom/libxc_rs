//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 938/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk938(t1426: f64, t27836: f64, t7063: f64, t7286: f64, t72: f64, t7929: f64, t686: f64, t7284: f64, t7289: f64, t1444: f64, t7296: f64, t7910: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27883 = t27836 * t1426;
    let t27884 = t7063 * t27883;
    let t27885 = t27884 * t7286;
    let t27887 = t7929 * t72;
    let t27888 = t27887 * t686;
    let t27889 = t7284 * t27888;
    let t27891 = t7289 * t27888;
    let t27896 = t7296 * t7910 * t1444;
    (t27883, t27884, t27885, t27888, t27889, t27891, t27896)
}
