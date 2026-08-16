//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3865/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3865(t22068: f64, t9765: f64, t22052: f64, t3989: f64, t1399: f64, t1410: f64, t22118: f64, t22274: f64, t3924: f64, t3934: f64, t4012: f64, t48798: f64, t73345: f64, t74269: f64, t74271: f64, t74277: f64, t74279: f64, t74281: f64, t74288: f64, t828: f64, t9955: f64) -> f64 {
    let t74290 = t9765 * t22068;
    let t74292 = t3989 * t22052;
    let t74298 = 7.0_f64 / 6.0_f64 * t74269 - 7.0_f64 / 12.0_f64 * t74271 - 0.42874018118069736972e-2_f64 * t3934 * t9955 * t22118 * t3924 - 0.22675591804667994221e-1_f64 * t74277 + 0.11337795902333997111e0_f64 * t74279 - 0.76220476654346199061e-4_f64 * t74281 + 0.51448821741683684366e-1_f64 * t3934 * t48798 * t22274 * t1399 - 0.4065600224742826258e-3_f64 * t74288 - 0.27104001498285508387e-2_f64 * t74290 + 0.80031500487063509014e-2_f64 * t74292 + 0.85748036236139473944e-2_f64 * t1410 * t4012 * t828 * t73345;
    t74298
}
