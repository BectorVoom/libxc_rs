//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 802/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk802(t24702: f64, t24756: f64, t466: f64, t24574: f64, t7368: f64, t2148: f64, t3477: f64, t1186: f64, t7381: f64, t3427: f64, t2121: f64, t225: f64, t24594: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24757 = t24702 + t24756;
    let t24758 = t466 * t24757;
    let t24760 = t24574 * t7368;
    let t24762 = t3477 * t2148;
    let t24765 = t1186 * t7381;
    let t24771 = t3427 * t2148;
    let t24773 = 0.18277045187202515961e-2_f64 * t2121 * t24771;
    let t24776 = t24594 * t225;
    (t24757, t24758, t24760, t24762, t24765, t24773, t24776)
}
