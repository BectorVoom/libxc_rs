//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 904/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk904<F: Float>(t16636: F, t3836: F, t16644: F, t2813: F, t14300: F, t14339: F, t1325: F, t8201: F, t16672: F, t241: F, t16824: F, t16826: F, t16885: F, t16931: F, t16935: F, t16941: F, t16945: F, t16947: F, t16955: F, t16957: F) -> (F, F, F, F, F, F, F) {
    let t17024 = t3836 * t16636;
    let t17028 = t2813 * t16644;
    let t17031 = t14300 * t14339;
    let t17034 = t8201 * t1325;
    let t17035 = t14300 * t17034;
    let t17039 = F::cast_from(0.19751789702565206229e-1_f64) * t241 * t16672;
    let t17040 = t16931 - t16935 - t16945 - t16941 - t16955 - t16957 + t16885 + t16947 + t17039 + t16824 + t16826;
    (t17024, t17028, t17031, t17034, t17035, t17039, t17040)
}
