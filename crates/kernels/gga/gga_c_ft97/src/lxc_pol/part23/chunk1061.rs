//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1061/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1061<F: Float>(t24260: F, t6: F, t3766: F, t230: F, t709: F, t420: F, t39: F, t9681: F, t294: F, t7639: F, t10363: F, t1113: F, t213: F, t1196: F, t2: F, t7242: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t33371 = t24260 * t6;
    let t33372 = t3766 * t33371;
    let t33373 = t230 * t709;
    let t33374 = t420 * t33373;
    let t33432 = t9681 * t39;
    let t33828 = 1.0 / t7639 / t294;
    let t33939 = t10363 * t39;
    let t35409 = t230 * t1113;
    let t35410 = t420 * t35409;
    let t35455 = t230 * t213;
    let t35456 = t420 * t35455;
    let t35877 = t230 * t1196;
    let t36452 = t7242 * t2;
    (t33372, t33374, t33432, t33828, t33939, t35409, t35410, t35455, t35456, t35877, t36452)
}
