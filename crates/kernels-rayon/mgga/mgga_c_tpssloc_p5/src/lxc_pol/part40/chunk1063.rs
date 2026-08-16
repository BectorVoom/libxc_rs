//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1063/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1063(t16935: f64, t4282: f64, t13433: f64, t1510: f64, t17030: f64, t829: f64, t13397: f64, t16817: f64, t16820: f64, t16823: f64, t16825: f64, t16828: f64, t16830: f64, t17023: f64, t17028: f64, t17031: f64, t17034: f64, t226: f64, t2617: f64, t4166: f64, t4281: f64, t4283: f64, t4288: f64, t4291: f64, t4292: f64, t5575: f64, t5651: f64, t5655: f64, t808: f64, t812: f64, t863: f64) -> f64 {
    let t17037 = t4282 * t16935;
    let t17041 = t13433 * t1510;
    let t17046 = t17030 * t829;
    let t17048 = -6.0_f64 * t13397 * t16817 + 4.0_f64 * t16820 * t4281 - t16823 * t812 + 6.0_f64 * t16825 * t4281 - t16828 * t4291 - 2.0_f64 * t16830 * t4292 + t17023 * t226 - t17028 * t812 + 2.0_f64 * t17031 * t4281 + 4.0_f64 * t17034 * t4283 + 4.0_f64 * t17037 * t4281 - 2.0_f64 * t17041 * t812 - t17046 * t4291 - t2617 * t5651 - 2.0_f64 * t4166 * t4288 + t5575 * t863 + t5655 * t808;
    t17048
}
