//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1333/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1333(t114172: f64, t22892: f64, t7691: f64, t114160: f64, t1985: f64, t7700: f64, t114174: f64, t22666: f64, t32697: f64, t3886: f64, t7749: f64, t1385: f64, t1992: f64, t22635: f64) -> (f64, f64, f64, f64, f64) {
    let t120308 = t22892 * t114172 * t7691;
    let t120309 = 0.16449340668482264365e-1_f64 * t120308;
    let t120312 = 0.16449340668482264365e-1_f64 * t1985 * t114160 * t7700;
    let t120313 = 0.82246703342411321825e-2_f64 * t114174;
    let t120316 = 0.16449340668482264365e-1_f64 * t1985 * t22666 * t32697;
    let t120317 = t3886 * t7749;
    let t120321 = 0.3289868133696452873e-1_f64 * t1992 * t22635 * t120317 * t1385;
    (t120309, t120312, t120313, t120316, t120321)
}
