//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1351/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1351<F: Float>(t112179: F, t112195: F, t112232: F, t112234: F, t112243: F, t112307: F, t1797: F, t1808: F, t24546: F, t24612: F, t24644: F, t24668: F, t24715: F, t24744: F, t24759: F, t24840: F, t26880: F, t29040: F, t7624: F, t97133: F, t97174: F, t97215: F, t97261: F) -> F {
    let t116134 = F::new(0.17149607247227894789e-2) * t112195 + F::new(0.17149607247227894789e-2) * t97174 * t24744 + F::new(0.12862205435420921092e-2) * t97261 * t24840 + F::new(0.95275595817932748825e-3) * t112232 + F::new(0.60976381323476959248e-2) * t112234 + F::new(0.14291339372689912324e-2) * t7624 * t24644 + F::new(0.43445671692977333464e-1) * t112307 * t1797 - F::new(0.25724410870841842183e-2) * t97215 * t24668 + F::new(0.42874018118069736972e-3) * t97133 * t24546 + F::new(0.17149607247227894789e-2) * t26880 * t24612 + F::new(0.85748036236139473944e-3) * t26880 * t24759 - F::new(0.11433071498151929859e-2) * t112243 + F::new(0.25724410870841842183e-2) * t29040 * t24715 - F::new(0.85748036236139473944e-3) * t112179 * t1808;
    t116134
}
