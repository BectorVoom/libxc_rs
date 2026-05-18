//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 974/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk974<F: Float>(t14586: F, t18444: F, t4364: F, t10756: F, t10758: F, t14780: F, t14817: F, t14820: F, t14839: F, t18350: F, t18354: F, t23281: F, t23285: F, t23289: F, t23293: F, t23297: F, t2745: F, t4362: F, t825: F, t851: F) -> (F, F) {
    let t23301 = t4364 * t18444 * t14586;
    let t23310 = F::new(0.12862205435420921092e-1) * t851 * t23281 - F::new(0.21437009059034868486e-3) * t825 * t23285 - F::new(0.21437009059034868486e-3) * t825 * t23289 + F::new(0.25724410870841842183e-2) * t2745 * t23293 - F::new(0.64311027177104605458e-3) * t2745 * t23297 + F::new(0.12862205435420921092e-2) * t4362 * t23301 + F::new(0.30492001685571196935e-4) * t14780 + F::new(0.85748036236139473944e-4) * t18350 - F::new(0.42874018118069736972e-3) * t18354 - F::new(0.5421477899694558815e-4) * t14817 + F::new(0.76230004213927992336e-5) * t14820 + F::new(0.16262400898971305032e-2) * t14839 - t10756 - t10758;
    (t23301, t23310)
}
