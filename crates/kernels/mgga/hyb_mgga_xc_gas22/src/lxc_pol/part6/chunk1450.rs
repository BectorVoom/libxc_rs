//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1450/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1450<F: Float>(t2874: F, t4540: F, t2880: F, t3748: F, t3951: F, t9645: F, t1117: F, t1134: F, t11447: F, t22809: F, t26403: F, t2869: F, t2876: F, t2889: F, t2893: F, t2903: F, t31496: F, t4524: F, t4544: F, t4553: F, t4556: F, t4559: F, t4562: F, t510: F, t518: F, t521: F, t7692: F, t7768: F, t7817: F, t9632: F) -> (F, F, F) {
    let t31631 = t2874 * t4540;
    let t31642 = t2880 * t4540;
    let t31651 = t3748 * t3951;
    let t31652 = t31651 * t9645;
    let t31655 = -F::cast_from(180.0_f64) * t2903 * t11447 * t2893 + F::cast_from(252.0_f64) * t1134 * t4556 * t2889 - F::cast_from(24.0_f64) * t510 * t7768 * t4544 * t2876 + F::cast_from(120.0_f64) * t510 * t521 * t4524 * t2876 + F::cast_from(360.0_f64) * t22809 * t4553 * t2869 + F::cast_from(252.0_f64) * t1134 * t31631 * t2876 - F::cast_from(180.0_f64) * t2903 * t4562 * t2869 - F::cast_from(336.0_f64) * t518 * t7692 * t4544 * t2876 + F::cast_from(12.0_f64) * t1117 * t31642 * t2876 + F::cast_from(120.0_f64) * t7817 * t4559 * t2869 + F::cast_from(2000.0_f64) * t26403 * t31496 + F::cast_from(704.0_f64) / F::cast_from(27.0_f64) * t9632 * t31652;
    (t31651, t31652, t31655)
}
