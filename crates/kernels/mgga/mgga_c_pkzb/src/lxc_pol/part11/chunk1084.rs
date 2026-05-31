//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1084/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1084<F: Float>(t11817: F, t204: F, t334: F, t1731: F, t218: F, t344: F, t5555: F, t847: F, t16194: F, t339: F, t930: F, t336: F) -> (F, F, F, F, F, F, F, F) {
    let t18439 = t204 * t11817 * t334;
    let t18440 = F::cast_from(0.31310740740740740741e1_f64) * t18439;
    let t18442 = t218 * t1731 * t344;
    let t18443 = F::cast_from(0.13490888888888888889e1_f64) * t18442;
    let t18445 = t218 * t5555 * t847;
    let t18468 = F::cast_from(280.0_f64) / F::cast_from(81.0_f64) * t18439;
    let t18480 = F::cast_from(1.0_f64) / t339 / t16194 / t930 / F::cast_from(96.0_f64);
    let t18492 = F::powf(t336, -F::cast_from(0.25e1_f64));
    (t18439, t18440, t18442, t18443, t18445, t18468, t18480, t18492)
}
