//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1446/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1446(t17794: f64, t3363: f64, t1042: f64, t372: f64, t5268: f64, t17695: f64, t13086: f64, t13090: f64, t13092: f64, t17693: f64, t17781: f64, t17786: f64, t17791: f64, t17792: f64, t3640: f64, t3644: f64, t3711: f64, t5331: f64, t5381: f64) -> f64 {
    let t17795 = t17794 * t3363;
    let t17796 = t1042 * t17795;
    let t17799 = t372 * t5268;
    let t17800 = t17799 * t17695;
    let t17803 = -0.14291339372689912324e-3_f64 * t5381 * t3640 - 0.95275595817932748827e-4_f64 * t13086 - 0.19055119163586549765e-3_f64 * t13090 - 0.19055119163586549765e-3_f64 * t13092 - 0.28582678745379824648e-3_f64 * t5381 * t3644 - 0.42874018118069736972e-3_f64 * t5331 * t17781 - 0.21437009059034868486e-3_f64 * t5331 * t17786 - t17791 + t17792 / 162.0_f64 - 0.23818898954483187207e-3_f64 * t3711 * t17796 - 0.57165357490759649296e-3_f64 * t17693 * t17800;
    t17803
}
