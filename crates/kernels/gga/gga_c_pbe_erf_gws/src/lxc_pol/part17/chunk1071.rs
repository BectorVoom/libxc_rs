//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1071/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1071<F: Float>(t13890: F, t4414: F, t1477: F, t274: F, t833: F, t850: F, t851: F, t14145: F, t2053: F, t13767: F, t804: F, t321: F, t50825: F, t1167: F, t2423: F, t3324: F, t810: F) -> (F, F, F, F, F, F, F, F) {
    let t52027 = t4414 * t13890;
    let t52033 = t274 * t1477;
    let t52036 = t850 * t851 * t52033 * t833;
    let t52052 = t14145 * t2053;
    let t52056 = t804 * t13767;
    let t52061 = t321 * t50825;
    let t52763 = t1167 * t2423;
    let t52767 = t3324 * t810;
    (t52027, t52033, t52036, t52052, t52056, t52061, t52763, t52767)
}
