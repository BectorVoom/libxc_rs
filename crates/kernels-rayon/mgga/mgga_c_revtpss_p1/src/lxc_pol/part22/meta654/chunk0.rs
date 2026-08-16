//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2604/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2604(t1246: f64, t20819: f64, t1214: f64, t5819: f64, t5302: f64, t1042: f64, t1252: f64, t1261: f64, t12809: f64, t17547: f64, t1797: f64, t20784: f64, t20787: f64, t20789: f64, t20792: f64, t20797: f64, t20802: f64, t20806: f64, t20811: f64, t20817: f64, t3711: f64, t5331: f64, t5340: f64) -> (f64, f64, f64, f64, f64) {
    let t20820 = t20819 * t1246;
    let t20823 = t5819 * t1214;
    let t20824 = t5302 * t20823;
    let t20825 = t1042 * t20824;
    let t20828 = 0.19055119163586549765e-3_f64 * t20784 - 0.14291339372689912324e-3_f64 * t20787 - 0.15244095330869239812e-2_f64 * t20789 + 0.23818898954483187207e-3_f64 * t1261 * t20792 + 0.21437009059034868486e-3_f64 * t12809 * t20797 + 0.42874018118069736972e-3_f64 * t5340 * t20802 - 0.21437009059034868486e-3_f64 * t5331 * t20806 + 0.14291339372689912324e-3_f64 * t3711 * t20811 - 0.22866142996303859718e-2_f64 * t17547 * t1797 + 0.14291339372689912324e-3_f64 * t20817 + 0.21437009059034868486e-3_f64 * t20820 * t1252 - 0.23818898954483187207e-3_f64 * t3711 * t20825;
    (t20820, t20823, t20824, t20825, t20828)
}
