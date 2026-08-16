//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 685/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk685(t1532: f64, t4871: f64, t1485: f64, t557: f64, t1531: f64, t1639: f64, t466: f64, t1626: f64, t496: f64, t1541: f64, t495: f64, t127: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4872 = t4871 * t1532;
    let t4874 = t1485 * t557;
    let t4876 = 0.21687162600603479684e-1_f64 * t1531 * t4874;
    let t4877 = t466 * t1639;
    let t4879 = 0.32530743900905219526e-1_f64 * t1531 * t4877;
    let t4880 = t496 * t1626;
    let t4881 = 12.0_f64 * t4880;
    let t4882 = t495 * t1541;
    let t4883 = t4882 * t127;
    (t4872, t4874, t4876, t4877, t4879, t4880, t4881, t4882, t4883)
}
