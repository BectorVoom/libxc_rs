//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1291/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1291<F: Float>(t17571: F, t3411: F, t1445: F, t1562: F, t2293: F, t8097: F, t2854: F, t6393: F, t10448: F, t4953: F, t10374: F, t4614: F, t574: F) -> (F, F, F, F, F) {
    let t34025 = F::cast_from(0.69017266717057349418e1_f64) * t17571 * t3411;
    let t34032 = F::cast_from(0.13803453343411469884e2_f64) * t1562 * t1445 * t8097 * t2293;
    let t34036 = F::cast_from(0.69017266717057349418e1_f64) * t1562 * t1445 * t2854 * t6393;
    let t34038 = F::cast_from(0.13803453343411469884e2_f64) * t4953 * t10448;
    let t34052 = F::cast_from(0.12269736305254639897e2_f64) * t574 * t4614 * t10374;
    (t34025, t34032, t34036, t34038, t34052)
}
