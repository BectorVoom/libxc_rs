//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1241/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1241<F: Float>(t1246: F, t20819: F, t1214: F, t5819: F, t5302: F, t1042: F, t1252: F, t1261: F, t12809: F, t17547: F, t1797: F, t20784: F, t20787: F, t20789: F, t20792: F, t20797: F, t20802: F, t20806: F, t20811: F, t20817: F, t3711: F, t5331: F, t5340: F) -> (F, F) {
    let t20820 = t20819 * t1246;
    let t20823 = t5819 * t1214;
    let t20824 = t5302 * t20823;
    let t20825 = t1042 * t20824;
    let t20828 = 0.19055119163586549765e-3 * t20784 - 0.14291339372689912324e-3 * t20787 - 0.15244095330869239812e-2 * t20789 + 0.23818898954483187207e-3 * t1261 * t20792 + 0.21437009059034868486e-3 * t12809 * t20797 + 0.42874018118069736972e-3 * t5340 * t20802 - 0.21437009059034868486e-3 * t5331 * t20806 + 0.14291339372689912324e-3 * t3711 * t20811 - 0.22866142996303859718e-2 * t17547 * t1797 + 0.14291339372689912324e-3 * t20817 + 0.21437009059034868486e-3 * t20820 * t1252 - 0.23818898954483187207e-3 * t3711 * t20825;
    (t20823, t20828)
}
