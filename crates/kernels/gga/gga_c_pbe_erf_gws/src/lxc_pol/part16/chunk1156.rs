//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1156/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1156<F: Float>(t1205: F, t3306: F, t2409: F, t3067: F, t4216: F, t8734: F, t1105: F, t4110: F, t2376: F, t14185: F, t3060: F, t9283: F) -> (F, F, F, F, F, F, F) {
    let t14943 = t1205 * t3306;
    let t14945 = t2409 * t3067 * t14943;
    let t14949 = t2409 * t8734 * t4216;
    let t14952 = t4110 * t1105;
    let t14954 = t2409 * t2376 * t14952;
    let t14958 = t14185 * t3060;
    let t14959 = t9283 * t14958;
    (t14943, t14945, t14949, t14952, t14954, t14958, t14959)
}
