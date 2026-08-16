//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2916/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2916(t23535: f64, t2880: f64, t918: f64, t51914: f64, t51915: f64, t63240: f64, t63242: f64, t77663: f64, t77667: f64, t77670: f64, t77672: f64, t77674: f64, t77676: f64) -> (f64, f64) {
    let t77679 = t2880 * t23535 * t918;
    let t77681 = t51914 - 0.91983333333333333333e-1_f64 * t51915 - 0.11038e0_f64 * t77663 + 0.99342e0_f64 * t63240 - 0.66228e0_f64 * t63242 + 0.24528888888888888889e-1_f64 * t77667 - 0.82785e-1_f64 * t77670 - 0.1237865625e0_f64 * t77672 + 0.247573125e0_f64 * t77674 + 0.247573125e0_f64 * t77676 - 0.1294625e1_f64 * t77679;
    (t77679, t77681)
}
