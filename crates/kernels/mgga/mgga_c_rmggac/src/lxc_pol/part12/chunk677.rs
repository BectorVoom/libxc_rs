//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 677/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk677<F: Float>(t352: F, t8975: F, t8946: F, t321: F, t333: F, t1587: F, t665: F, t305: F, t326: F, t4669: F, t5148: F, t5259: F, t5266: F, t7826: F, t7832: F, t7842: F, t8958: F, t8960: F, t8963: F, t8966: F, t8971: F, t8973: F) -> (F, F) {
    let t8976 = t8975 * t352;
    let t8979 = t8946 * t352;
    let t8982 = t8975 * t321;
    let t8985 = t8975 * t333;
    let t8988 = t665 * t1587;
    let t8991 = F::new(0.19957069503106347607e-1) * t8958 - F::new(0.59871208509319042821e-1) * t326 * t8960 - F::new(0.11974241701863808564e0) * t5148 * t8963 - F::new(0.44903406381989282115e-1) * t8966 + F::new(0.27274661654245341729e-1) * t7826 - F::new(0.36366215538993788972e-1) * t7832 - F::new(0.90915538847484472429e-2) * t7842 + F::new(0.2993560425465952141e-1) * t8971 - F::new(0.2993560425465952141e-1) * t8973 - F::new(0.11974241701863808564e0) * t5148 * t8976 + F::new(0.11974241701863808564e0) * t5266 * t8979 + F::new(0.11974241701863808564e0) * t5259 * t8982 - F::new(0.17961362552795712846e0) * t4669 * t8985 + F::new(0.59871208509319042821e-1) * t305 * t8988;
    (t8988, t8991)
}
