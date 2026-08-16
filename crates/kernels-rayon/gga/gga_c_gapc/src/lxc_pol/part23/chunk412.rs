//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 412/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk412(t1254: f64, t1257: f64, t1261: f64, t414: f64, t491: f64, t2053: f64, t2058: f64, t2059: f64, t2078: f64, t2141: f64, t257: f64, t260: f64, t266: f64, t738: f64, t748: f64, t751: f64, t758: f64) -> (f64, f64) {
    let t2150 = -0.15474205398478635379e-1_f64 * t414 + 0.5833205e-2_f64 * t1254 - 0.16123583333333333333e-2_f64 * t1257 + 0.61251011229312867192e-4_f64 * t491 - 0.6735290625e-5_f64 * t1261;
    let t2152 = 0.21272952746160294864e-2_f64 * t414 * t257 + 0.42545905492320589728e-2_f64 * t2053 * t748 + 0.63818858238480884592e-2_f64 * t2058 * t2059 - 0.21272952746160294864e-2_f64 * t738 * t2078 - t2141 * t266 - 2.0_f64 * t751 * t758 - t260 * t2150;
    (t2150, t2152)
}
