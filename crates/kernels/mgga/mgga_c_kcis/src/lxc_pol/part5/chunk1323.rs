//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1323/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1323<F: Float>(t1307: F, t21886: F, t1395: F, t1394: F, t1387: F, t15989: F, t16628: F, t16629: F, t16632: F, t16663: F, t17287: F, t20953: F, t21848: F, t21852: F, t21856: F, t21861: F, t21865: F, t21868: F, t21872: F, t21874: F, t21879: F, t21881: F, t21884: F, t3961: F, t5742: F, t5886: F) -> (F, F) {
    let t21887 = t21886 * t1307;
    let t21888 = t1395 * t21887;
    let t21889 = t1394 * t21888;
    let t21893 = -t15989 + t16628 - F::cast_from(0.58958024691358024689e-2_f64) * t16629 - t16632 + F::new(0.13345e0) * t5742 * t5886 + F::cast_from(0.178089025e-1_f64) * t17287 * t5886 - F::cast_from(0.22109259259259259259e-2_f64) * t21848 - F::cast_from(0.22109259259259259258e-2_f64) * t21852 - F::cast_from(0.7369753086419753086e-3_f64) * t21856 - F::cast_from(0.44218518518518518516e-2_f64) * t21861 + F::cast_from(0.3684876543209876543e-2_f64) * t21865 - F::cast_from(0.7369753086419753086e-3_f64) * t16663 - F::new(0.66725e-1) * t21868 * t1387 + F::cast_from(0.27636574074074074073e-2_f64) * t21872 + F::cast_from(0.14739506172839506172e-2_f64) * t21874 + F::cast_from(0.99491666666666666664e-2_f64) * t21879 - F::cast_from(0.22109259259259259259e-2_f64) * t21881 - F::cast_from(0.88437037037037037034e-2_f64) * t21884 + F::cast_from(0.1621345679012345679e-1_f64) * t21889 + F::cast_from(0.178089025e-1_f64) * t3961 * t20953;
    (t21889, t21893)
}
