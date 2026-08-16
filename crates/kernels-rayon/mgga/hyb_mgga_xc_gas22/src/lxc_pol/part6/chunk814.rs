//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 814/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk814(t1145: f64, t4544: f64, t2880: f64, t4524: f64, t4530: f64, t521: f64, t2874: f64, t4540: f64, t1139: f64, t513: f64, t1128: f64, t502: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4545 = t1145 * t4544;
    let t4550 = t2880 * t4524;
    let t4553 = t521 * t4530;
    let t4556 = t2874 * t4524;
    let t4559 = t521 * t4540;
    let t4562 = t1139 * t4544;
    let t4565 = t513 * t4540;
    let t4568 = t1128 * t4544;
    let t4571 = t502 * t4530;
    (t4545, t4550, t4553, t4556, t4559, t4562, t4565, t4568, t4571)
}
