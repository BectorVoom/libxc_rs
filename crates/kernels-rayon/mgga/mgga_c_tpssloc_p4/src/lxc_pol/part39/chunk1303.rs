//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1303/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1303(t2585: f64, t656: f64, t1849: f64, t8189: f64, t2199: f64, t5361: f64, t1266: f64, t8273: f64, t1774: f64, t29895: f64, t8262: f64, t26129: f64, t8180: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t30175 = t2585 * t656;
    let t30266 = t8189 * t1849;
    let t30269 = t2199 * t5361;
    let t30272 = t1266 * t8273;
    let t30274 = t1774 * t8189;
    let t30279 = t29895 * t8262;
    let t30281 = t8180 * t26129;
    (t30175, t30266, t30269, t30272, t30274, t30279, t30281)
}
