//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1229/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1229(t6897: f64, t8458: f64, t90544: f64, t114154: f64, t114172: f64, t22892: f64, t7691: f64, t114160: f64, t1985: f64, t7700: f64, t114174: f64, t22666: f64, t32697: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t120296 = t6897 * t90544 * t8458;
    let t120297 = 0.82246703342411321825e-2_f64 * t120296;
    let t120304 = 0.82246703342411321825e-2_f64 * t114154;
    let t120308 = t22892 * t114172 * t7691;
    let t120309 = 0.16449340668482264365e-1_f64 * t120308;
    let t120312 = 0.16449340668482264365e-1_f64 * t1985 * t114160 * t7700;
    let t120313 = 0.82246703342411321825e-2_f64 * t114174;
    let t120316 = 0.16449340668482264365e-1_f64 * t1985 * t22666 * t32697;
    (t120297, t120304, t120309, t120312, t120313, t120316)
}
