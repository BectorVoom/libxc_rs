//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1314/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1314<F: Float>(t2843: F, t4299: F, t7124: F, t1501: F, t19862: F, t111624: F, t111625: F, t111783: F, t112439: F, t112449: F, t112452: F, t112479: F, t1466: F, t1506: F, t18497: F, t18514: F, t18989: F, t18992: F, t193: F, t19460: F, t28944: F, t28945: F, t29000: F, t5422: F, t6216: F, t6222: F, t6967: F, t824: F, t98268: F) -> (F, F, F) {
    let t125742 = t2843 * t7124 * t4299;
    let t125745 = t2843 * t1501 * t19862;
    let t125764 = -t112479 * t6967 / 9.0 - t18989 * t1506 - t112439 - t1466 * t193 * t6222 * t5422 * t824 / 3.0 + 8.0 * t125742 + 4.0 * t125745 - t18992 * t1506 + t112449 + t112452 + 2.0 / 27.0 * t6216 * t98268 * t28945 * t19460 + 2.0 / 9.0 * t6216 * t28944 * t111783 * t18514 - 5.0 / 81.0 * t6216 * t111624 * t111625 * t18514 + 4.0 / 27.0 * t29000 * t28944 * t28945 * t18497;
    (t125742, t125745, t125764)
}
