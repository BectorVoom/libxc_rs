//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 946/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk946(t225: f64, t9849: f64, t9850: f64, t9852: f64, t9869: f64, t4010: f64, t73: f64, t9400: f64, t3889: f64, t9737: f64, t1394: f64, t9628: f64) -> (f64, f64, f64, f64) {
    let t9872 = (t9849 + t9850 + t9852 + t9869) * t225;
    let t9880 = t73 * t4010;
    let t9881 = t9880 * t9400;
    let t9884 = t9737 * t3889;
    let t9887 = t1394 * t9628;
    (t9872, t9881, t9884, t9887)
}
