//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1168/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1168(t34340: f64, t34347: f64, t30330: f64, t30334: f64, t30343: f64, t30347: f64, t30349: f64, t30353: f64, t30355: f64, t30362: f64, t32435: f64, t34327: f64, t34330: f64, t34336: f64, t34343: f64, t34349: f64, t34351: f64) -> f64 {
    let t37034 = 0.13719685797782315831e-1_f64 * t34340;
    let t37036 = 0.28582678745379824648e-3_f64 * t34347;
    let t37043 = 0.22921875e-1_f64 * t34327 + 0.4584375e-1_f64 * t34330 - 0.42874018118069736972e-2_f64 * t30330 - 0.17149607247227894789e-2_f64 * t30334 + t32435 + 0.62896184579208304138e-3_f64 * t34336 + 0.21437009059034868486e-2_f64 * t30343 + 0.85748036236139473944e-3_f64 * t30347 + t37034 - 0.21437009059034868486e-3_f64 * t34343 - t37036 - 0.75475421495049964966e-2_f64 * t34349 + 0.75475421495049964966e-2_f64 * t34351 - 0.18868855373762491241e-2_f64 * t30349 + 0.20965394859736101379e-3_f64 * t30353 - 0.21437009059034868486e-3_f64 * t30355 - 0.14291339372689912324e-3_f64 * t30362;
    t37043
}
