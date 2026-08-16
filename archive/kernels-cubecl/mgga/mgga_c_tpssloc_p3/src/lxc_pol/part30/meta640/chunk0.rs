//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2050/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2050<F: Float>(t25580: F, t3053: F, t23529: F, t4571: F, t13961: F, t6755: F, t14202: F, t6765: F, t13950: F, t23422: F, t4603: F, t14159: F, t6717: F) -> (F, F, F, F, F, F, F) {
    let t88305 = t25580 * t3053 / F::cast_from(1728.0_f64);
    let t88307 = t23529 * t4571 / F::cast_from(324.0_f64);
    let t88320 = t6755 * t13961 / F::cast_from(1152.0_f64);
    let t88321 = t6765 * t14202;
    let t88324 = t6765 * t13950 / F::cast_from(1728.0_f64);
    let t88335 = t23422 * t4603 / F::cast_from(162.0_f64);
    let t88336 = t6717 * t14159;
    (t88305, t88307, t88320, t88321, t88324, t88335, t88336)
}
