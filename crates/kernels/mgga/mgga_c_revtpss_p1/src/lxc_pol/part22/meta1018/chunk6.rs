//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3527/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3527<F: Float>(t15794: F, t15926: F, t1011: F, t15993: F, t18937: F, t11875: F, t15785: F, t15906: F, t16081: F, t19450: F, t19639: F, t20089: F, t3117: F, t42571: F, t43279: F, t4912: F, t4915: F, t53586: F, t54623: F, t54638: F, t54646: F, t54648: F, t54916: F, t6263: F, t6271: F, t63297: F) -> F {
    let t66814 = t15926 * t15794;
    let t66822 = t1011 * t15993 * t18937;
    let t66827 = F::cast_from(0.11433071498151929859e-2_f64) * t54623 - F::cast_from(0.12862205435420921092e-2_f64) * t15906 * t3117 * t19450 * t15785 + F::cast_from(0.85748036236139473944e-3_f64) * t11875 * t3117 * t20089 * t19639 + F::cast_from(0.42874018118069736972e-3_f64) * t11875 * t3117 * t6271 * t53586 + F::cast_from(0.12862205435420921092e-2_f64) * t16081 * t3117 * t19450 * t43279 + F::cast_from(0.45732285992607719436e-2_f64) * t54916 * t4912 - F::cast_from(0.57165357490759649296e-3_f64) * t66814 + F::cast_from(0.6351706387862183255e-3_f64) * t54638 + F::cast_from(0.30488190661738479624e-2_f64) * t42571 * t6263 - F::cast_from(0.96545937095505185476e-2_f64) * t54646 - F::cast_from(0.10162730220579493208e-2_f64) * t54648 + t66822 / F::cast_from(324.0_f64) - t1011 * t4915 * t63297 / F::cast_from(144.0_f64);
    t66827
}
