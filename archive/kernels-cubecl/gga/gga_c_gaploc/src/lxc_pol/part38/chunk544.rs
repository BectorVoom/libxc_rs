//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 544/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk544<F: Float>(t4752: F, t888: F, t2859: F, t10314: F, t6717: F, t6716: F, t6711: F, t6710: F, t9333: F, t3410: F, t4614: F, t1562: F) -> (F, F, F, F, F, F) {
    let t10348 = t4752 * t888;
    let t10350 = F::cast_from(0.7150097990370085334e0_f64) * t2859 * t10348;
    let t10351 = t6717 * t10314;
    let t10353 = F::cast_from(0.69017266717057349418e1_f64) * t6716 * t10351;
    let t10354 = t6711 * t10314;
    let t10356 = F::cast_from(0.11502877786176224903e2_f64) * t6710 * t10354;
    let t10358 = F::cast_from(0.10725146985555128001e1_f64) * t2859 * t9333;
    let t10359 = t4614 * t3410;
    let t10361 = F::cast_from(0.92023022289409799224e1_f64) * t1562 * t10359;
    (t10348, t10350, t10353, t10356, t10358, t10361)
}
