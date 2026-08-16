//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1315/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1315(t4139: f64, t6564: f64, t2189: f64, t6562: f64, t10625: f64, t10626: f64, t10631: f64, t10667: f64, t10672: f64, t20730: f64, t20741: f64, t20744: f64, t2291: f64, t2292: f64, t2307: f64, t2315: f64, t2322: f64, t3430: f64, t4180: f64, t4193: f64, t4214: f64, t6640: f64, t6737: f64, t856: f64, t8600: f64, t8601: f64, t8608: f64, t8743: f64) -> (f64, f64) {
    let t28741 = t4139 * t6564;
    let t28744 = 0.51726012919273400301e3_f64 * t6562 * t28741 * t2189;
    let t28779 = t28744 - 0.91082604192152556044e5_f64 * t856 * t20741 * t4180 * t20744 * t2291 + 0.10389515463408878255e3_f64 * t856 * t6640 * t4193 * t2315 - 0.35089341735807877242e1_f64 * t856 * t4214 * t2307 - 0.20508037716432813316e4_f64 * t2322 * t10631 + 0.46785788981077169656e1_f64 * t3430 * t8743 + 0.10389515463408878255e3_f64 * t856 * t10667 * t6737 + 0.12304822629859687989e5_f64 * t856 * t20730 * t4180 * t8600 + 0.23392894490538584828e1_f64 * t2322 * t10672 - 0.34631718211362927517e2_f64 * t3430 * t8608 - 0.20508037716432813315e4_f64 * t3430 * t8601 - 0.35089341735807877242e1_f64 * t856 * t10625 * t2292 - 0.34631718211362927518e2_f64 * t2322 * t10626;
    (t28744, t28779)
}
