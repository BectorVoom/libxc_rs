//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1116/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1116<F: Float>(t14011: F, t9588: F, t14498: F, t9353: F, t14567: F, t2080: F, t9544: F, t9365: F, t2134: F, t8897: F, t51267: F, t8983: F, t14007: F, t9334: F, t51470: F, t9338: F) -> (F, F, F, F, F, F, F, F) {
    let t54177 = t14011 * t9588;
    let t54179 = t14498 * t9353;
    let t54183 = t2080 * t9544 * t14567;
    let t54186 = t2080 * t9365 * t14567;
    let t54188 = t2134 * t8897;
    let t54190 = t51267 * t8983;
    let t54192 = t14007 * t9334;
    let t54194 = t51470 * t9338;
    (t54177, t54179, t54183, t54186, t54188, t54190, t54192, t54194)
}
