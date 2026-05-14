//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 993/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk993<F: Float>(t10906: F, t4929: F, t17556: F, t10925: F, t10972: F, t10978: F, t10983: F, t11119: F, t1706: F, t1735: F, t17497: F, t17516: F, t17520: F, t17523: F, t17530: F, t17533: F, t17536: F, t17540: F, t17543: F, t17547: F, t17553: F, t2418: F, t45: F, t4853: F, t4858: F, t4860: F, t4909: F, t4931: F, t634: F, t7096: F, t7135: F, t7139: F, t7151: F) -> (F,) {
    let t17557 = t10906 * t4929;
    let t17558 = t17556 * t17557;
    let t17561 = 1.0 * t10972 * t2418 + 2.0 * t4853 * t7135 + 1.0 * t1706 * t17497 + 0.19751789702565206229e-1 * t45 * t17516 * t634 - 2.0 * t17520 * t4860 + 6.0 * t4909 * t17523 - 4.0 * t11119 * t7096 + 0.32163648644302209644e2 * t10978 * t7139 - 4.0 * t4858 * t17530 - 2.0 * t4858 * t17533 - 0.96490945932906628932e2 * t10983 * t17536 + 0.32163648644302209644e2 * t4909 * t17540 + 0.16081824322151104822e2 * t4909 * t17543 + 0.51725014705706168417e3 * t10925 * t17547 + 0.11696446794910408142e1 * t7151 * t4931 + 0.23392893589820816284e1 * t1735 * t17553 - 0.1025389702100779493e4 * t1735 * t17558;
    (t17561,)
}
