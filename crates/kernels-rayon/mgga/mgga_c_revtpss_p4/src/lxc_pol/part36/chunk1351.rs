//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1351/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1351(t112179: f64, t112195: f64, t112232: f64, t112234: f64, t112243: f64, t112307: f64, t1797: f64, t1808: f64, t24546: f64, t24612: f64, t24644: f64, t24668: f64, t24715: f64, t24744: f64, t24759: f64, t24840: f64, t26880: f64, t29040: f64, t7624: f64, t97133: f64, t97174: f64, t97215: f64, t97261: f64) -> f64 {
    let t116134 = 0.17149607247227894789e-2_f64 * t112195 + 0.17149607247227894789e-2_f64 * t97174 * t24744 + 0.12862205435420921092e-2_f64 * t97261 * t24840 + 0.95275595817932748825e-3_f64 * t112232 + 0.60976381323476959248e-2_f64 * t112234 + 0.14291339372689912324e-2_f64 * t7624 * t24644 + 0.43445671692977333464e-1_f64 * t112307 * t1797 - 0.25724410870841842183e-2_f64 * t97215 * t24668 + 0.42874018118069736972e-3_f64 * t97133 * t24546 + 0.17149607247227894789e-2_f64 * t26880 * t24612 + 0.85748036236139473944e-3_f64 * t26880 * t24759 - 0.11433071498151929859e-2_f64 * t112243 + 0.25724410870841842183e-2_f64 * t29040 * t24715 - 0.85748036236139473944e-3_f64 * t112179 * t1808;
    t116134
}
