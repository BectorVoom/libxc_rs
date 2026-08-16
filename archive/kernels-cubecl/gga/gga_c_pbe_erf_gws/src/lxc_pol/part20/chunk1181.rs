//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1181/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1181<F: Float>(t15204: F, t3983: F, t1192: F, t3703: F, t2376: F, t2409: F, t4155: F, t8589: F, t2503: F, t4127: F, t3863: F, t4039: F) -> (F, F, F, F, F, F) {
    let t15205 = t3983 * t15204;
    let t15207 = t1192 * t3703;
    let t15209 = t2409 * t2376 * t15207;
    let t15213 = t2409 * t8589 * t4155;
    let t15216 = t4127 * t2503;
    let t15218 = t4039 * t3863;
    (t15205, t15207, t15209, t15213, t15216, t15218)
}
