//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 781/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk781<F: Float>(t4300: F, t939: F, t2492: F, t4296: F, t946: F, t1386: F, t238: F, t242: F, t341: F, t4283: F, t2489: F, t2499: F, t3478: F, t3520: F, t4285: F, t4297: F) -> (F, F, F, F, F, F, F, F) {
    let t4301 = t939 * t4300;
    let t4305 = t2492 * t4296;
    let t4307 = t946 * t4300;
    let t4310 = t1386 * t1386;
    let t4312 = t238 * t242 * t4310;
    let t4314 = t341 * t4283;
    let t4316 = t238 * t242 * t4314;
    let t4318 = -0.9494625e0 * t4297 + 0.1898925e1 * t4301 + t2489 - 0.59793333333333333334e0 * t3478 + 0.8969e0 * t4285 + 0.15358125e0 * t4305 + 0.3071625e0 * t4307 + t2499 - 0.32862666666666666666e0 * t3520 + 0.24647e0 * t4312 + 0.24647e0 * t4316;
    (t4301, t4305, t4307, t4310, t4312, t4314, t4316, t4318)
}
