//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 644/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk644<F: Float>(t352: F, t8975: F, t8946: F, t321: F, t333: F, t1587: F, t665: F, t305: F, t326: F, t4669: F, t5148: F, t5259: F, t5266: F, t7826: F, t7832: F, t7842: F, t8958: F, t8960: F, t8963: F, t8966: F, t8971: F, t8973: F) -> (F, F) {
    let t8976 = t8975 * t352;
    let t8979 = t8946 * t352;
    let t8982 = t8975 * t321;
    let t8985 = t8975 * t333;
    let t8988 = t665 * t1587;
    let t8991 = F::cast_from(0.19957069503106347607e-1_f64) * t8958 - F::cast_from(0.59871208509319042821e-1_f64) * t326 * t8960 - F::cast_from(0.11974241701863808564e0_f64) * t5148 * t8963 - F::cast_from(0.44903406381989282115e-1_f64) * t8966 + F::cast_from(0.27274661654245341729e-1_f64) * t7826 - F::cast_from(0.36366215538993788972e-1_f64) * t7832 - F::cast_from(0.90915538847484472429e-2_f64) * t7842 + F::cast_from(0.2993560425465952141e-1_f64) * t8971 - F::cast_from(0.2993560425465952141e-1_f64) * t8973 - F::cast_from(0.11974241701863808564e0_f64) * t5148 * t8976 + F::cast_from(0.11974241701863808564e0_f64) * t5266 * t8979 + F::cast_from(0.11974241701863808564e0_f64) * t5259 * t8982 - F::cast_from(0.17961362552795712846e0_f64) * t4669 * t8985 + F::cast_from(0.59871208509319042821e-1_f64) * t305 * t8988;
    (t8988, t8991)
}
