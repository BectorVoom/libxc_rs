//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 943/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk943(t1307: f64, t21886: f64, t1395: f64, t1394: f64, t1387: f64, t15989: f64, t16628: f64, t16629: f64, t16632: f64, t16663: f64, t17287: f64, t20953: f64, t21848: f64, t21852: f64, t21856: f64, t21861: f64, t21865: f64, t21868: f64, t21872: f64, t21874: f64, t21879: f64, t21881: f64, t21884: f64, t3961: f64, t5742: f64, t5886: f64) -> (f64, f64) {
    let t21887 = t21886 * t1307;
    let t21888 = t1395 * t21887;
    let t21889 = t1394 * t21888;
    let t21893 = -t15989 + t16628 - 0.58958024691358024689e-2_f64 * t16629 - t16632 + 0.13345e0_f64 * t5742 * t5886 + 0.178089025e-1_f64 * t17287 * t5886 - 0.22109259259259259259e-2_f64 * t21848 - 0.22109259259259259258e-2_f64 * t21852 - 0.7369753086419753086e-3_f64 * t21856 - 0.44218518518518518516e-2_f64 * t21861 + 0.3684876543209876543e-2_f64 * t21865 - 0.7369753086419753086e-3_f64 * t16663 - 0.66725e-1_f64 * t21868 * t1387 + 0.27636574074074074073e-2_f64 * t21872 + 0.14739506172839506172e-2_f64 * t21874 + 0.99491666666666666664e-2_f64 * t21879 - 0.22109259259259259259e-2_f64 * t21881 - 0.88437037037037037034e-2_f64 * t21884 + 0.1621345679012345679e-1_f64 * t21889 + 0.178089025e-1_f64 * t3961 * t20953;
    (t21889, t21893)
}
