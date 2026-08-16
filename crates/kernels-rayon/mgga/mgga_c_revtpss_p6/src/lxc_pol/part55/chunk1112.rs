//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1112/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1112(t34790: f64, t34795: f64, t34800: f64, t34827: f64, t3: f64, t1918: f64, t2115: f64, t2170: f64, t34011: f64, t34014: f64, t34346: f64, t34348: f64, t34350: f64, t34358: f64, t34362: f64, t34365: f64, t34368: f64, t573: f64, t8124: f64, t8127: f64, t8245: f64, t8616: f64, t8905: f64, param_d: f64) -> (f64, f64, f64, f64) {
    let t34829 = t34790 + t34795 + t34800 + t34827;
    let t34830 = t3 * t34829;
    let t34838 = param_d * t34829;
    let t34848 = 3.0_f64 * t1918 * t8905 + 3.0_f64 * t2115 * t8245 + 6.0_f64 * t2170 * t8124 + 3.0_f64 * t2170 * t8127 + t34838 * t573 + t34011 + t34014 + t34346 + t34348 + t34350 + t34358 + t34362 + t34365 + t34368 + t8616;
    (t34829, t34830, t34838, t34848)
}
