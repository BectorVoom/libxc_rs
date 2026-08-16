//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2869/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2869(t13655: f64, t4354: f64, t41811: f64, t5695: f64, t4471: f64, t17488: f64, t892: f64, t914: f64, t10771: f64, t10811: f64, t14271: f64, t14328: f64, t14460: f64, t14466: f64, t17547: f64, t17554: f64, t2861: f64, t2862: f64, t2880: f64, t2886: f64, t2905: f64, t42154: f64, t42226: f64, t42228: f64, t4437: f64, t49263: f64, t5742: f64, t5759: f64, t59941: f64, t59958: f64, t59961: f64, t59962: f64, t59966: f64, t59968: f64, t933: f64, t951: f64) -> (f64, f64, f64, f64, f64) {
    let t59970 = 2.0_f64 * t4354 * t13655;
    let t59972 = 2.0_f64 * t41811 * t5695;
    let t59975 = t4471 * t4471;
    let t59979 = t17488 * t892;
    let t59981 = 2.0_f64 * t59979 * t914;
    let t59982 = -0.24828486201251232145e5_f64 * t42154 * t17554 * t2862 - 2.0_f64 * t2861 * t5759 * t2880 - 0.19298375398431042081e3_f64 * t10771 * t17547 * t2862 + 0.32163958997385070134e2_f64 * t2886 * t17547 * t2880 + 0.2069040516770936012e4_f64 * t10811 * t59941 * t2862 + 0.64327917994770140268e2_f64 * t2886 * t4437 * t14328 + 0.2069040516770936012e4_f64 * t10811 * t17554 * t2880 + 0.19964560303604640732e6_f64 * t42226 * t5742 * t42228 * t2862 - 0.4155806185363551302e3_f64 * t49263 * t14460 - t59958 - t59961 + 2.0_f64 * t59962 * t933 - t59966 - t59968 - t59970 + t59972 + 12.0_f64 * t14271 * t14466 - 0.23392894490538584828e1_f64 * t2905 * t59975 * t951 - t59981;
    (t59970, t59972, t59975, t59981, t59982)
}
