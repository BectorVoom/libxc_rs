//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1145/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1145<F: Float>(t2134: F, t8897: F, t51267: F, t8983: F, t14007: F, t9334: F, t51470: F, t9338: F, t14498: F, t9671: F, t14028: F, t3299: F, t14567: F, t6608: F, t9484: F, t14535: F, t2115: F) -> (F, F, F, F, F, F, F, F) {
    let t54188 = t2134 * t8897;
    let t54190 = t51267 * t8983;
    let t54192 = t14007 * t9334;
    let t54194 = t51470 * t9338;
    let t54196 = t14498 * t9671;
    let t54198 = t14028 * t3299;
    let t54199 = 7.0 / 576.0 * t54198;
    let t54201 = t6608 * t9484 * t14567;
    let t54203 = t2115 * t14535;
    (t54188, t54190, t54192, t54194, t54196, t54199, t54201, t54203)
}
