//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 874/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk874<F: Float>(t8072: F, t895: F, t4752: F, t888: F, t2859: F, t10314: F, t6717: F, t6716: F, t6711: F, t6710: F, t9333: F, t3410: F, t4614: F, t1562: F, t3411: F, t4953: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t10336 = 0.35750489951850426669e0 * t895 * t8072;
    let t10348 = t4752 * t888;
    let t10350 = 0.7150097990370085334e0 * t2859 * t10348;
    let t10351 = t6717 * t10314;
    let t10353 = 0.69017266717057349418e1 * t6716 * t10351;
    let t10354 = t6711 * t10314;
    let t10356 = 0.11502877786176224903e2 * t6710 * t10354;
    let t10358 = 0.10725146985555128001e1 * t2859 * t9333;
    let t10359 = t4614 * t3410;
    let t10361 = 0.92023022289409799224e1 * t1562 * t10359;
    let t10363 = 0.69017266717057349418e1 * t4953 * t3411;
    (t10336, t10348, t10350, t10351, t10353, t10354, t10356, t10358, t10359, t10361, t10363)
}
