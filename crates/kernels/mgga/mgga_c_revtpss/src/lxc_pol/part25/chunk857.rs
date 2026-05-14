//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 857/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk857<F: Float>(t10725: F, t10791: F, t10848: F, t10908: F, t136: F, t860: F, t2457: F, t2710: F, t10519: F, t10524: F, t10533: F, t10539: F, t10543: F, t10548: F, t10639: F, t10645: F, t10647: F, t10651: F, t10655: F, t10657: F, t10661: F, t10666: F, t213: F, t234: F, t2646: F, t2724: F, t2815: F, t820: F, t837: F, t879: F) -> (F, F) {
    let t10910 = t10725 + t10791 + t10848 + t10908;
    let t10914 = t860 * t136;
    let t10916 = t2710 * t10914 * t2457;
    let t10918 = 0.39029762157531132076e-1 * t10519 - 0.29272321618148349057e-1 * t10524 - 0.19756347548806534796e1 * t820 * t2815 * t2646 + 0.58544643236296698113e-1 * t10533 - 0.34697458558045176417e-2 * t10539 - 0.58544643236296698113e-1 * t10543 - 0.29272321618148349057e-1 * t10548 - 0.65854491829355115987e0 * t820 * t879 * t10639 - t10645 - 0.39029762157531132076e-1 * t10647 + t10651 - 0.32927245914677557992e-1 * t10655 - 0.19756347548806534796e1 * t820 * t10657 * t837 + 0.39512695097613069591e1 * t820 * t10661 * t2724 - 0.65854491829355115987e0 * t820 * t879 * t10666 + 0.65854491829355115987e0 * t213 * t234 * t10910 + 0.34697458558045176417e-2 * t10916;
    (t10910, t10918)
}
