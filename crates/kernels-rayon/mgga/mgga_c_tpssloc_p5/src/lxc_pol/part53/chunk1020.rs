//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1020/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1020(t114680: f64, t114691: f64, t114752: f64, t116686: f64, t116688: f64, t121541: f64, t121546: f64, t121550: f64, t121560: f64, t121563: f64, t121574: f64, t121612: f64, t123584: f64, t123626: f64, t1499: f64, t226: f64, t235: f64, t2617: f64, t31993: f64, t31996: f64, t33969: f64, t33971: f64, t4162: f64, t4182: f64, t4234: f64, t4281: f64, t808: f64, t812: f64, t8738: f64) -> f64 {
    let t123663 = -t2617 * t33969 + 0.16449340668482264365e-1_f64 * t114680 - t116686 - 0.16449340668482264365e-1_f64 * t114691 + t116688 + 2.0_f64 * t4281 * t123626 * t4182 + 0.6579736267392905746e-1_f64 * t121541 - 0.3289868133696452873e-1_f64 * t121546 + 0.3289868133696452873e-1_f64 * t121550 - t812 * t31993 * t4234 + t4162 * t8738 + t808 * t33971 - 0.3289868133696452873e-1_f64 * t121560 - 0.3289868133696452873e-1_f64 * t121563 + 0.76763589786250567037e-1_f64 * t114752 - 0.76763589786250567037e-1_f64 * t121574 + 0.19739208802178717238e0_f64 * t121612 + t226 * t235 * t123584 + t1499 * t31996;
    t123663
}
