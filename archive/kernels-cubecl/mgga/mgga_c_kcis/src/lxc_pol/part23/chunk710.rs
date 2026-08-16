//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 710/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk710<F: Float>(t1928: F, t20: F, t251: F, t1592: F, t1889: F, t7979: F, t1600: F, t2104: F, t7984: F, t6176: F, t2260: F, t7968: F, t7976: F, t7978: F, t7991: F, t8166: F, t8169: F, t8172: F, t8177: F, t8180: F, t8209: F, t8213: F) -> (F, F, F, F, F, F, F) {
    let t8217 = t251 * t1928 * t20;
    let t8218 = t1592 * t8217;
    let t8221 = t7979 * t1889;
    let t8222 = t1600 * t8221;
    let t8225 = t7984 * t2104;
    let t8226 = t6176 * t8225;
    let t8236 = -F::cast_from(0.34752604166666666667e-3_f64) * t8209 * t2260 + F::cast_from(0.46377350260416666667e-4_f64) * t7968 * t8213 + F::cast_from(0.92673611111111111112e-3_f64) * t8218 * t2260 - t7976 - F::cast_from(0.11584201388888888889e-3_f64) * t7978 * t8222 + F::cast_from(0.34752604166666666667e-3_f64) * t7978 * t8226 + F::cast_from(0.34752604166666666667e-3_f64) * t7978 * t8213 + t7991 + F::cast_from(0.11607361111111111111e-2_f64) * t8166 + F::cast_from(0.17411041666666666666e-2_f64) * t8169 - F::cast_from(0.17411041666666666666e-2_f64) * t8172 - F::cast_from(0.46429444444444444443e-2_f64) * t8177 + F::cast_from(0.11607361111111111111e-2_f64) * t8180;
    (t8217, t8218, t8221, t8222, t8225, t8226, t8236)
}
