//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2942/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2942<F: Float>(t16082: F, t999: F, t3155: F, t3133: F, t4900: F, t11875: F, t11927: F, t15893: F, t15907: F, t15917: F, t1592: F, t15973: F, t15975: F, t16067: F, t16076: F, t19620: F, t19639: F, t3092: F, t3117: F, t357: F, t42240: F, t42249: F, t42251: F, t42550: F, t42621: F, t43050: F, t4583: F, t4899: F) -> (F, F, F) {
    let t53506 = t16082 * t999;
    let t53511 = t3155 * t999;
    let t53516 = t4900 * t3133;
    let t53528 = F::cast_from(0.64311027177104605458e-3_f64) * t11875 * t3117 * t16076 * t19639 - F::cast_from(0.42874018118069736972e-3_f64) * t15917 * t15975 - F::cast_from(0.42874018118069736972e-3_f64) * t4899 * t3092 * t4583 * t15973 + F::cast_from(0.14291339372689912324e-3_f64) * t16067 * t3092 * t1592 * t42550 * t357 - F::cast_from(0.38586616306262763275e-2_f64) * t42621 * t3117 * t15907 * t53506 + F::cast_from(0.25724410870841842183e-2_f64) * t43050 * t3117 * t15893 * t53511 + F::cast_from(0.64311027177104605458e-3_f64) * t16067 * t3117 * t15907 * t53516 + F::cast_from(0.12862205435420921092e-2_f64) * t11927 * t3117 * t16076 * t19620 + t42240 / F::new(54.0) - t42249 / F::new(108.0) - t42251 / F::new(81.0);
    (t53506, t53516, t53528)
}
