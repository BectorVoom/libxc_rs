//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1135/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1135<F: Float>(t2214: F, t3293: F, t528: F, t132: F, t1567: F, t7340: F, t1054: F, t6132: F, t7345: F, t6139: F, t37702: F, t37707: F, t37714: F, t39599: F, t39602: F, t39604: F, t39608: F, t39610: F) -> (F, F) {
    let t39613 = t3293 * t2214 * t528;
    let t39614 = t132 * t1567;
    let t39616 = t39613 * t39614 * t7340;
    let t39619 = t6132 * t1054 * t7345;
    let t39622 = t6139 * t1054 * t7340;
    let t39624 = -F::cast_from(0.97574405393827830186e-2_f64) * t37702 - F::cast_from(0.45022119329691164872e0_f64) * t37707 - F::cast_from(0.47609969197673950972e-2_f64) * t37714 + F::cast_from(0.21831846657716620896e-2_f64) * t39599 + t39602 - F::cast_from(0.43663693315433241792e-2_f64) * t39604 - t39608 - F::cast_from(0.86682217400542685632e-1_f64) * t39610 - F::cast_from(0.21951497276451705328e0_f64) * t39616 - F::cast_from(0.17336443480108537126e0_f64) * t39619 - F::cast_from(0.5200933044032561138e0_f64) * t39622;
    (t39613, t39624)
}
