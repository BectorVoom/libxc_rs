//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 792/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk792<F: Float>(t1479: F, t553: F, t535: F, t837: F, t551: F, t1371: F, t1952: F, t1378: F, t1971: F, t5697: F, t1960: F, t1464: F, t285: F, t545: F) -> (F, F, F, F, F, F) {
    let t6005 = F::new(0.258995450979035416e-1) * t1479 * t553;
    let t6006 = t837 * t535;
    let t6008 = t6006 * t551 * t553;
    let t6012 = F::new(0.19753890328909480882e-1) * t1952 * t1371 * t553;
    let t6015 = F::new(0.34679929861433484636e-2) * t5697 * t1378 * t1971;
    let t6021 = t1960 * t1371 * t553;
    let t6028 = F::new(0.40679438125041687114e-2) * t1464 * t545 * t285;
    (t6005, t6008, t6012, t6015, t6021, t6028)
}
