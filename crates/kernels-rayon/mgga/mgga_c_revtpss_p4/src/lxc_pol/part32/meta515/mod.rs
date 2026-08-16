//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta515 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1817;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta515(t30555: f64, t30625: f64, t3: f64, t2055: f64, t5883: f64, t1518: f64, t28986: f64, t5920: f64, t7553: f64, t117: f64, t30570: f64, t1916: f64, t1918: f64, t2113: f64, t2115: f64, t572: f64, t573: f64, t6941: f64, t6945: f64, t6948: f64, t8118: f64, t8124: f64, t8127: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t30626, t30627, t30637, t30651, t30654, t30657, t30660, t30663) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1817(t30555, t30625, t3, t2055, t5883, t1518, t28986, t5920, t7553, t117, t30570, t1916, t1918, t2113, t2115, t572, t573, t6941, t6945, t6948, t8118, t8124, t8127, param_d);
    (t30626, t30627, t30637, t30651, t30654, t30657, t30660, t30663)
}
