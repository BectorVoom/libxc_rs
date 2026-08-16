//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1411/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1411<F: Float>(t11265: F, t3271: F, t3279: F, t11243: F, t39267: F, t404: F, t410: F, t1100: F, t43832: F, t3270: F, t407: F, t3287: F) -> (F, F, F, F, F, F, F) {
    let t43872 = t11265 * t3271 * t3279;
    let t43875 = t11243 * t3271 * t3279;
    let t43880 = F::cast_from(1.0_f64) / t410 / t39267 / t404 / F::cast_from(96.0_f64);
    let t43881 = t3271 * t3271;
    let t43882 = t43880 * t43881;
    let t43884 = t1100 * t43832;
    let t43886 = t3279 * t3279;
    let t43887 = t3270 * t43886;
    let t43889 = F::powf(t407, -F::cast_from(0.25e1_f64));
    let t43890 = t43889 * t43881;
    let t43892 = t3287 * t43886;
    (t43872, t43875, t43882, t43884, t43887, t43890, t43892)
}
