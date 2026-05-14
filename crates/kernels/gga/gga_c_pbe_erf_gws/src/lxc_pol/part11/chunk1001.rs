//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1001/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1001<F: Float>(t1820: F, t1885: F, t41432: F, t995: F, t12544: F, t7130: F, t32670: F, t41359: F, t41385: F, t41388: F, t41395: F, t41398: F, t41401: F, t41404: F, t48092: F, t48095: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t48099 = 32.0 / 5.0 * t1820 * t1885 * t41432 * t995;
    let t48101 = 16.0 / 5.0 * t7130 * t12544;
    let t48102 = 16.0 / 45.0 * t32670;
    let t48103 = 32.0 / 15.0 * t41359;
    let t48104 = 64.0 / 45.0 * t41385;
    let t48105 = 128.0 / 45.0 * t41388;
    let t48106 = 128.0 / 45.0 * t41395;
    let t48107 = 32.0 / 15.0 * t41398;
    let t48108 = 16.0 / 45.0 * t41401;
    let t48109 = 64.0 / 27.0 * t41404;
    let t48110 = t48092 - t48095 - t48099 - t48101 - t48102 + t48103 - t48104 - t48105 - t48106 + t48107 - t48108 - t48109;
    (t48099, t48101, t48102, t48103, t48104, t48105, t48106, t48107, t48108, t48109, t48110)
}
