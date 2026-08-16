//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1051/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1051(t34691: f64, t34692: f64, t4263: f64, t30407: f64, t31097: f64, t495: f64, t7325: f64, t4410: f64, t7561: f64, t30543: f64, t8610: f64, t30934: f64, t8614: f64) -> (f64, f64, f64, f64, f64) {
    let t34694 = t34691 * t34692 * t4263;
    let t34698 = t30407 * t31097 * t7325 * t495;
    let t34700 = t7561 * t4410;
    let t34702 = t30543 * t8610;
    let t34704 = t30934 * t8614;
    (t34694, t34698, t34700, t34702, t34704)
}
