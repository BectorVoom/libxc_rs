//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 967/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk967<F: Float>(t18113: F, t19103: F, t19170: F, t19469: F, t312: F, t16386: F, t18424: F, t18428: F, t18432: F, t18435: F, t18439: F, t18441: F, t18445: F, t18448: F, t18452: F, t18456: F, t18460: F, t18462: F, t18467: F, t2182: F, t2423: F, t2424: F, t2429: F, t321: F) -> (F, F) {
    let t19472 = (t18113 + t19103 + t19170 + t19469) * t312;
    let t19476 = 12.0 * t16386 * t2423 * t321 + 36.0 * t2182 * t2424 * t2429 + t18424 - t18428 + t18432 - t18435 + t18439 - t18441 - t18445 - t18448 - t18452 + t18456 - t18460 - t18462 + t18467 - t19472;
    (t19472, t19476)
}
