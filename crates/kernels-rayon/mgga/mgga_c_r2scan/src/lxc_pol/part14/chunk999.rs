//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 999/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk999(t11036: f64, t2381: f64, t2391: f64, t3358: f64, t1070: f64, t8355: f64, t3363: f64, t8358: f64, t2378: f64, t3366: f64, t3629: f64, t6654: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11868 = t11036 * t2381;
    let t11870 = t3358 * t2391;
    let t11872 = t8355 * t1070;
    let t11874 = t8358 * t3363;
    let t11876 = t2378 * t3366;
    let t11878 = t6654 * t3629;
    (t11868, t11870, t11872, t11874, t11876, t11878)
}
