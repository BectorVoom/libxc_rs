//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1234/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1234(t40889: f64, t68: f64, t252: f64, t9957: f64, t2678: f64, t852: f64, t225: f64, t9520: f64, t1022: f64, t2250: f64, t11018: f64, t11016: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40890 = t68 * t40889;
    let t40909 = t252 * t9957;
    let t40955 = t852 * t2678;
    let t41554 = t9520 * t225;
    let t43240 = t2250 * t1022;
    let t43431 = t11018 * t225;
    let t43440 = t11016 * t225;
    (t40890, t40909, t40955, t41554, t43240, t43431, t43440)
}
