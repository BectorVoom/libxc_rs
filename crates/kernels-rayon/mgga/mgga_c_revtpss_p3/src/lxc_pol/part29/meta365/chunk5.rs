//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1317/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1317(t1843: f64, t2371: f64, t1310: f64, t4292: f64, t1518: f64, t3813: f64, t5517: f64, t670: f64, t13514: f64, t508: f64, t10416: f64, t13435: f64, t13517: f64, t1453: f64, t1502: f64, t1519: f64, t2322: f64, t2328: f64, t2372: f64, t4248: f64, t4254: f64, t4257: f64, t4293: f64, t4297: f64, t5528: f64, t569: f64, t651: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13521 = t1843 * t2371;
    let t13532 = t1310 * t4292;
    let t13537 = t3813 * t1518;
    let t13540 = t5517 * t670;
    let t13544 = t508 * t13514;
    let t13547 = -2.0_f64 * t10416 * t1519 - 4.0_f64 * t13435 * t1519 + t13517 * t569 - 2.0_f64 * t13521 * t651 - 4.0_f64 * t13532 * t651 - 2.0_f64 * t13537 * t651 - 4.0_f64 * t13540 * t651 - 2.0_f64 * t13544 * t651 + 2.0_f64 * t1453 * t5528 - t1502 * t3813 - 2.0_f64 * t1843 * t2328 - 4.0_f64 * t2322 * t4257 - 4.0_f64 * t2322 * t4297 - 2.0_f64 * t2372 * t4248 - 4.0_f64 * t4254 * t4293;
    (t13521, t13532, t13537, t13540, t13544, t13547)
}
