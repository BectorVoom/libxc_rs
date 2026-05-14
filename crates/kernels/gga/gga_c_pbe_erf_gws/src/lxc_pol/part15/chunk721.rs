//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 721/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk721<F: Float>(t589: F, t597: F, t187: F, t190: F, t5044: F, t1860: F, t401: F, t1251: F, t607: F, t1863: F, t1857: F, t177: F, t572: F, t191: F, t4939: F, t1740: F, t579: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5219 = t589 * t597;
    let t5241 = 0.29629629629629629629e-1 * t190 * t5044 * t187;
    let t5248 = t401 * t1860;
    let t5256 = t1251 * t607;
    let t5258 = t401 * t1863;
    let t5260 = t401 * t1857;
    let t5263 = 1.0 / t177 / t572;
    let t5264 = t191 * t5263;
    let t5271 = 0.11197407407407407407e0 * t4939;
    let t5278 = t579 * t1740;
    (t5219, t5241, t5248, t5256, t5258, t5260, t5264, t5271, t5278)
}
