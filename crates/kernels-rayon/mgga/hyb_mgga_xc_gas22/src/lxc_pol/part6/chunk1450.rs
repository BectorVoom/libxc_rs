//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1450/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1450(t2874: f64, t4540: f64, t2880: f64, t3748: f64, t3951: f64, t9645: f64, t1117: f64, t1134: f64, t11447: f64, t22809: f64, t26403: f64, t2869: f64, t2876: f64, t2889: f64, t2893: f64, t2903: f64, t31496: f64, t4524: f64, t4544: f64, t4553: f64, t4556: f64, t4559: f64, t4562: f64, t510: f64, t518: f64, t521: f64, t7692: f64, t7768: f64, t7817: f64, t9632: f64) -> (f64, f64, f64) {
    let t31631 = t2874 * t4540;
    let t31642 = t2880 * t4540;
    let t31651 = t3748 * t3951;
    let t31652 = t31651 * t9645;
    let t31655 = -180.0_f64 * t2903 * t11447 * t2893 + 252.0_f64 * t1134 * t4556 * t2889 - 24.0_f64 * t510 * t7768 * t4544 * t2876 + 120.0_f64 * t510 * t521 * t4524 * t2876 + 360.0_f64 * t22809 * t4553 * t2869 + 252.0_f64 * t1134 * t31631 * t2876 - 180.0_f64 * t2903 * t4562 * t2869 - 336.0_f64 * t518 * t7692 * t4544 * t2876 + 12.0_f64 * t1117 * t31642 * t2876 + 120.0_f64 * t7817 * t4559 * t2869 + 2000.0_f64 * t26403 * t31496 + 704.0_f64 / 27.0_f64 * t9632 * t31652;
    (t31651, t31652, t31655)
}
