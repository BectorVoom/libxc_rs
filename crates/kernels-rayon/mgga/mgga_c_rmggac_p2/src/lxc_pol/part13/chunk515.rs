//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 515/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk515(t1228: f64, t1522: f64, t4555: f64, t608: f64, t1392: f64, t6: f64, t1515: f64, t446: f64, t1477: f64, t4559: f64, t1193: f64, t5582: f64) -> (f64, f64, f64, f64, f64) {
    let t5681 = 0.25610252642437845428e0_f64 * t1228 * t1522;
    let t5685 = t4555 * t608;
    let t5687 = t6 * t1392;
    let t5689 = t1515 * t5687 * t446;
    let t5693 = 0.25610252642437845428e0_f64 * t4559 * t1477;
    let t5694 = t1193 * t5582;
    (t5681, t5685, t5689, t5693, t5694)
}
