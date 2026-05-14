//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 835/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk835<F: Float>(t23279: F, t2477: F, t828: F, t23177: F, t827: F, t23245: F, t18426: F, t2747: F, t6035: F, t4364: F, t4365: F, t6017: F, t14586: F, t18444: F, t10756: F, t10758: F, t14780: F, t14817: F, t14820: F, t14839: F, t18350: F, t18354: F, t2745: F, t4362: F, t825: F, t851: F) -> (F, F, F, F, F, F, F) {
    let t23281 = t2477 * t828 * t23279;
    let t23285 = t827 * t828 * t23177;
    let t23289 = t827 * t828 * t23245;
    let t23293 = t2747 * t18426 * t6035;
    let t23297 = t4364 * t4365 * t6017;
    let t23301 = t4364 * t18444 * t14586;
    let t23310 = 0.12862205435420921092e-1 * t851 * t23281 - 0.21437009059034868486e-3 * t825 * t23285 - 0.21437009059034868486e-3 * t825 * t23289 + 0.25724410870841842183e-2 * t2745 * t23293 - 0.64311027177104605458e-3 * t2745 * t23297 + 0.12862205435420921092e-2 * t4362 * t23301 + 0.30492001685571196935e-4 * t14780 + 0.85748036236139473944e-4 * t18350 - 0.42874018118069736972e-3 * t18354 - 0.5421477899694558815e-4 * t14817 + 0.76230004213927992336e-5 * t14820 + 0.16262400898971305032e-2 * t14839 - t10756 - t10758;
    (t23281, t23285, t23289, t23293, t23297, t23301, t23310)
}
