//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1057/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1057(t117357: f64, t117359: f64, t124603: f64, t124609: f64, t124612: f64, t124635: f64, t124668: f64, t1396: f64, t1398: f64, t1852: f64, t2099: f64, t27286: f64, t32311: f64, t34102: f64, t5364: f64, t7223: f64, t7240: f64, t7946: f64, t7961: f64, t8822: f64) -> f64 {
    let t124671 = 2.0_f64 * t7946 * t7240 + 2.0_f64 * t124603 + 2.0_f64 * t7223 * t7961 + 2.0_f64 * t2099 * t27286 + t117357 + t117359 + t124609 + t5364 * t8822 + t1852 * t32311 + t124612 + t1396 * t34102 + t1398 * (t124635 + t124668);
    t124671
}
