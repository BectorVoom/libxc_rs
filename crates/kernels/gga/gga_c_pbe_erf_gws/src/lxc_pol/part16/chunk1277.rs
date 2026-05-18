//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1277/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1277<F: Float>(t4028: F, t9135: F, t14015: F, t9655: F, t51421: F, t9490: F, t14011: F, t9588: F, t14498: F, t9353: F, t14567: F, t2080: F, t9544: F) -> (F, F, F, F, F, F) {
    let t54170 = t4028 * t9135;
    let t54173 = t14015 * t9655;
    let t54175 = t51421 * t9490;
    let t54177 = t14011 * t9588;
    let t54179 = t14498 * t9353;
    let t54183 = t2080 * t9544 * t14567;
    (t54170, t54173, t54175, t54177, t54179, t54183)
}
