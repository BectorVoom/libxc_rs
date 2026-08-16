//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 903/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk903(t10725: f64, t10791: f64, t10848: f64, t10908: f64, t136: f64, t860: f64, t2457: f64, t2710: f64, t10519: f64, t10524: f64, t10533: f64, t10539: f64, t10543: f64, t10548: f64, t10639: f64, t10645: f64, t10647: f64, t10651: f64, t10655: f64, t10657: f64, t10661: f64, t10666: f64, t213: f64, t234: f64, t2646: f64, t2724: f64, t2815: f64, t820: f64, t837: f64, t879: f64) -> (f64, f64) {
    let t10910 = t10725 + t10791 + t10848 + t10908;
    let t10914 = t860 * t136;
    let t10916 = t2710 * t10914 * t2457;
    let t10918 = 0.39029762157531132076e-1_f64 * t10519 - 0.29272321618148349057e-1_f64 * t10524 - 0.19756347548806534796e1_f64 * t820 * t2815 * t2646 + 0.58544643236296698113e-1_f64 * t10533 - 0.34697458558045176417e-2_f64 * t10539 - 0.58544643236296698113e-1_f64 * t10543 - 0.29272321618148349057e-1_f64 * t10548 - 0.65854491829355115987e0_f64 * t820 * t879 * t10639 - t10645 - 0.39029762157531132076e-1_f64 * t10647 + t10651 - 0.32927245914677557992e-1_f64 * t10655 - 0.19756347548806534796e1_f64 * t820 * t10657 * t837 + 0.39512695097613069591e1_f64 * t820 * t10661 * t2724 - 0.65854491829355115987e0_f64 * t820 * t879 * t10666 + 0.65854491829355115987e0_f64 * t213 * t234 * t10910 + 0.34697458558045176417e-2_f64 * t10916;
    (t10910, t10918)
}
