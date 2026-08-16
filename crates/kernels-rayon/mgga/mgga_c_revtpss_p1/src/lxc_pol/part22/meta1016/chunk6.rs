//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3514/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3514(t11710: f64, t16089: f64, t19706: f64, t16095: f64, t20095: f64, t1011: f64, t11675: f64, t11703: f64, t15599: f64, t16102: f64, t20079: f64, t3091: f64, t3162: f64, t42328: f64, t42710: f64, t43082: f64, t43085: f64, t4915: f64, t4919: f64, t54142: f64, t54147: f64, t6092: f64, t63244: f64, t63248: f64, t63306: f64, t63353: f64, t66187: f64) -> f64 {
    let t66467 = t16089 * t11710 * t19706;
    let t66470 = t16095 * t11710 * t20095;
    let t66500 = 0.76220476654346199061e-3_f64 * t66467 + 0.76220476654346199061e-3_f64 * t66470 + 0.28582678745379824648e-3_f64 * t11675 * t20079 + 0.23818898954483187207e-3_f64 * t3091 * t11703 * t6092 * t15599 + 0.57165357490759649296e-3_f64 * t54142 - 0.19055119163586549765e-3_f64 * t54147 + 0.28582678745379824648e-3_f64 * t42328 * t66187 * t3162 * t16102 + t1011 * t4915 * t63244 / 48.0_f64 + t1011 * t4919 * t63306 / 6.0_f64 - t1011 * t4915 * t63248 / 72.0_f64 - t1011 * t4919 * t63353 / 36.0_f64 - 0.31758531939310916275e-4_f64 * t42710 - 0.57165357490759649296e-3_f64 * t43082 * t66187 * t43085;
    t66500
}
