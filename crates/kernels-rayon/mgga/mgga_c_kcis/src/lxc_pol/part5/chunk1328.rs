//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1328/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1328(t1466: f64, t7192: f64, t1490: f64, t1464: f64, t1498: f64, t20961: f64, t3728: f64, t6924: f64, t6929: f64, t3738: f64, t7203: f64, t10443: f64, t18431: f64, t19653: f64, t8: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21955 = t7192 * t1466;
    let t21956 = t21955 * sigma2;
    let t21957 = t21956 * t1490;
    let t21958 = t1464 * t21957;
    let t21960 = t20961 * t1498;
    let t21961 = t1464 * t21960;
    let t21963 = t3728 * t6924;
    let t21965 = t3728 * t6929;
    let t21967 = t3738 * t7203;
    let t21968 = t1464 * t21967;
    let t21971 = t18431 * t8 - t10443 - t19653;
    (t21955, t21958, t21961, t21963, t21965, t21968, t21971)
}
