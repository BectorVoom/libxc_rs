//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta421 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1483;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta421(t31653: f64, t569: f64, t1911: f64, t8406: f64, t1843: f64, t2198: f64, t6934: f64, t6765: f64, t508: f64, t1312: f64, t18245: f64, t2199: f64, t2201: f64, t29508: f64, t30138: f64, t30143: f64, t4248: f64, t651: f64, t7732: f64, t7889: f64, t8393: f64, t8407: f64, t8411: f64, t8413: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t31654, t31657, t31660, t31663, t31674, t31677, t31700) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1483(t31653, t569, t1911, t8406, t1843, t2198, t6934, t6765, t508, t1312, t18245, t2199, t2201, t29508, t30138, t30143, t4248, t651, t7732, t7889, t8393, t8407, t8411, t8413);
    (t31654, t31657, t31660, t31663, t31674, t31677, t31700)
}
