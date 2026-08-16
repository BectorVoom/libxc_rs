//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1330/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1330(t10194: f64, t1310: f64, t1453: f64, t2163: f64, t2328: f64, t26804: f64, t27066: f64, t27079: f64, t4151: f64, t4254: f64, t508: f64, t7683: f64, t7687: f64, t95066: f64, t95068: f64, t95070: f64, t95073: f64, t95075: f64, t95081: f64, t95083: f64, t95085: f64, t95087: f64, t95090: f64, t95096: f64, t95104: f64, t95108: f64, t96709: f64) -> f64 {
    let t97565 = -6.0_f64 * t10194 * t2163 - 6.0_f64 * t1310 * t26804 + 3.0_f64 * t1453 * t27066 - 6.0_f64 * t2328 * t7683 - 6.0_f64 * t27079 * t4254 + 3.0_f64 * t4151 * t7687 - 6.0_f64 * t508 * t96709 - t95066 - t95068 - t95070 - t95073 - t95075 + t95081 - t95083 - t95085 - t95087 - t95090 + t95096 - t95104 + t95108;
    t97565
}
