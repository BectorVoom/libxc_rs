//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1074/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1074(t30226: f64, t30230: f64, t30233: f64, t30239: f64, t30240: f64, t30243: f64, t30247: f64, t30249: f64, t33963: f64, t33983: f64, t33987: f64, t33995: f64, t36876: f64, t36889: f64, t38890: f64, t38894: f64, t38899: f64, t38903: f64) -> f64 {
    let t38905 = 0.22921875e-1_f64 * t38890 + 0.1528125e-1_f64 * t38894 - t36876 + t33963 + 0.85748036236139473944e-3_f64 * t30226 + t30230 + t30233 + t30239 + 0.10718504529517434243e-3_f64 * t30240 + t30243 - t30247 - 0.45351183609335988444e-1_f64 * t30249 + 0.42874018118069736972e-3_f64 * t38899 - t33983 + t36889 + t33987 + 0.18868855373762491241e-2_f64 * t38903 + t33995;
    t38905
}
