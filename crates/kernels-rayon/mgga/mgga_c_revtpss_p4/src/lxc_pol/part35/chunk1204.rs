//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1204/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1204(t102378: f64, t102386: f64, t108282: f64, t109631: f64, t109633: f64, t109647: f64, t109651: f64, t114485: f64, t114621: f64, t115166: f64, t2097: f64, t2103: f64, t22974: f64, t25930: f64, t26304: f64, t27837: f64, t28899: f64, t30227: f64, t30279: f64, t6919: f64, t7295: f64, t8100: f64, t94656: f64, t94683: f64, t96401: f64, t9994: f64) -> f64 {
    let t115209 = 0.43368140941025997312e-1_f64 * t109631 - 0.77108554593144223218e-1_f64 * t109633 + 0.10408353825846239354e2_f64 * t7295 * t94656 * t2097 * t22974 - 0.38554277296572111609e-1_f64 * t109647 + 0.58544643236296698113e-1_f64 * t109651 - 0.26020884564615598386e1_f64 * t27837 * t30227 - 0.4336814094102599731e0_f64 * t114485 * t2103 - 0.51405703062096148812e-1_f64 * t102378 + t96401 + 0.68549505033305214441e-2_f64 * t102386 - 0.26020884564615598386e1_f64 * t25930 * t26304 * t114621 - 0.19756347548806534796e1_f64 * t28899 * t6919 + 0.13010442282307799193e1_f64 * t108282 * t8100 - 0.78062653693846795158e1_f64 * t27837 * t30279 + 0.26020884564615598386e1_f64 * t7295 * t94683 * t115166 * t9994;
    t115209
}
