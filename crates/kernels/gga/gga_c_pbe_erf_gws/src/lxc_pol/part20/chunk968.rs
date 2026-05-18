//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 968/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk968<F: Float>(t10917: F, t1820: F, t1627: F, t3500: F, t1648: F, t3504: F, t3522: F, t5480: F, t639: F, t1630: F, t3518: F, t3512: F, t5493: F) -> (F, F, F, F, F, F) {
    let t10919 = F::new(8.0) / F::new(15.0) * t1820 * t10917;
    let t10921 = F::new(8.0) / F::new(45.0) * t1627 * t3500;
    let t10923 = F::new(8.0) / F::new(45.0) * t1648 * t3504;
    let t10924 = t5480 * t3522;
    let t10925 = t639 * t10924;
    let t10926 = F::new(8.0) / F::new(81.0) * t10925;
    let t10927 = t1630 * t3518;
    let t10928 = t639 * t10927;
    let t10929 = F::new(8.0) / F::new(135.0) * t10928;
    let t10930 = t5493 * t3512;
    (t10919, t10921, t10923, t10926, t10929, t10930)
}
