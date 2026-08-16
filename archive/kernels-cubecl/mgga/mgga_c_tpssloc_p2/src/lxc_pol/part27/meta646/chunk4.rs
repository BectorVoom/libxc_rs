//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2223/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2223<F: Float>(t7554: F, t82632: F, t14529: F, t14545: F, t23327: F, t23341: F, t23346: F, t23395: F, t25406: F, t25413: F, t25732: F, t25784: F, t3016: F, t3026: F, t349: F, t388: F, t4660: F, t6687: F, t6816: F, t7553: F, t7565: F, t82437: F, t82463: F, t82490: F, t83296: F, t83303: F, t88728: F) -> F {
    let t88731 = t82632 * t7554;
    let t88742 = -F::cast_from(0.14621636149762012769e-1_f64) * t82437 - F::cast_from(2.0_f64) * t3026 * t25732 + F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t3016 * t25784 + F::cast_from(0.43864908449286038306e-1_f64) * t23346 * t25413 + F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t25406 * t23395 - F::cast_from(2.0_f64) * t14545 * t6816 + F::cast_from(0.27415567780803773942e-2_f64) * t82463 - F::cast_from(6.0_f64) * t4660 * t23341 + t349 * t88728 * t388 - F::cast_from(0.60923483957341719871e-3_f64) * t88731 - F::cast_from(2.0_f64) * t14529 * t6816 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t83296 * t7565 - F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t83303 * t7553 + F::cast_from(0.12184696791468343974e-2_f64) * t82490;
    t88742
}
