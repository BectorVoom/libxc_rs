//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 875/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk875(t2142: f64, t3566: f64, t26936: f64, t7642: f64, t7635: f64, t1209: f64, t7627: f64, t460: f64, t3555: f64, t1204: f64, t3801: f64, t7669: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26976 = t3566 * t2142;
    let t26979 = t7642 * t26936;
    let t26994 = t3566 * t7635;
    let t26999 = t1209 * t7627;
    let t27008 = t460 * t7627;
    let t27011 = t3555 * t2142;
    let t27020 = t1204 * t2142;
    let t27025 = t1209 * t26936;
    let t27037 = t7669 * t3801;
    (t26976, t26979, t26994, t26999, t27008, t27011, t27020, t27025, t27037)
}
