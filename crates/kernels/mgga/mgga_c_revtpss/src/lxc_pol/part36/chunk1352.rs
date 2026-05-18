//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1352/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1352<F: Float>(t104888: F, t112258: F, t112260: F, t112279: F, t112301: F, t112322: F, t112356: F, t1808: F, t24535: F, t24569: F, t24573: F, t24640: F, t26880: F, t29037: F, t29097: F, t29100: F, t6640: F, t6673: F, t6679: F, t6683: F, t7624: F) -> F {
    let t116160 = -F::new(0.17149607247227894789e-2) * t104888 * t6640 + F::new(0.91464571985215438873e-2) * t112260 * t1808 - F::new(0.57165357490759649295e-3) * t112258 - F::new(0.28963781128651555642e-1) * t112356 * t1808 + F::new(0.14291339372689912324e-2) * t29037 * t6673 - F::new(0.85748036236139473944e-3) * t29037 * t6679 - F::new(0.17149607247227894789e-2) * t29037 * t6683 - F::new(0.11433071498151929859e-2) * t112279 - F::new(0.14291339372689912324e-2) * t26880 * t24640 - F::new(0.1270341277572436651e-2) * t7624 * t24535 + F::new(11.0) / F::new(108.0) * t112301 + F::new(0.85748036236139473944e-3) * t112322 - F::new(0.17149607247227894789e-2) * t29097 * t24569 + F::new(0.85748036236139473944e-3) * t29100 * t24573;
    t116160
}
