//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1612/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1612(t12256: f64, t3698: f64, t1012: f64, t1042: f64, t1222: f64, t12800: f64, t12816: f64, t225: f64, t3600: f64, t3604: f64, t3620: f64, t3647: f64, t3692: f64, t39443: f64, t39449: f64, t44283: f64, t44286: f64, t44289: f64, t44291: f64, t44293: f64, t44321: f64, t44326: f64, t44333: f64, t44343: f64, t44346: f64, t480: f64, t484: f64) -> f64 {
    let t44348 = t3698 * t12256;
    let t44353 = -0.17149607247227894789e-2_f64 * t44283 - 0.19055119163586549765e-2_f64 * t44286 - 0.22866142996303859719e-2_f64 * t44289 + 0.2540682555144873302e-3_f64 * t44291 - 0.28582678745379824648e-3_f64 * t44293 + 0.21437009059034868486e-3_f64 * t44321 * t225 * t480 * t484 + 0.57165357490759649296e-3_f64 * t44326 + 0.14291339372689912324e-2_f64 * t12800 * t3620 + 0.57165357490759649296e-2_f64 * t3647 * t12816 + 0.12862205435420921092e-2_f64 * t3600 * t1042 * t44333 * t3604 - t1222 * t1012 * t3692 * t39449 / 48.0_f64 + t44343 / 108.0_f64 + t44346 / 27.0_f64 + t1222 * t1012 * t44348 * t39443 / 6.0_f64;
    t44353
}
