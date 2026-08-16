//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2112/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2112(t13826: f64, t7271: f64, t13923: f64, t7264: f64, t14036: f64, t25997: f64, t13946: f64, t26028: f64, t94456: f64, t94460: f64, t98161: f64, t98165: f64, t98169: f64, t98170: f64, t98172: f64, t98174: f64) -> f64 {
    let t98176 = t7271 * t13826;
    let t98178 = t7264 * t13923;
    let t98180 = t25997 * t14036;
    let t98181 = 0.50820002809285328226e-4_f64 * t98180;
    let t98182 = t26028 * t13946;
    let t98184 = 0.50820002809285328225e-5_f64 * t98161 - 0.40015750243531754508e-2_f64 * t94456 - 0.22675591804667994222e-1_f64 * t94460 - 0.45351183609335988442e-1_f64 * t98165 - t98169 + t98170 / 8.0_f64 + t98172 / 16.0_f64 + 0.27104001498285508387e-3_f64 * t98174 - 0.51448821741683684367e-1_f64 * t98176 - 0.42874018118069736972e-3_f64 * t98178 - t98181 - 0.85748036236139473944e-3_f64 * t98182;
    t98184
}
