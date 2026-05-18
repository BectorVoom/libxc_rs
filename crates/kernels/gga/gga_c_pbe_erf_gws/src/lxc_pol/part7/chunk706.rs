//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 706/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk706<F: Float>(t1552: F, t19: F, t506: F, t299: F, t481: F, t799: F, t119: F, t1533: F, t155: F, t1557: F, t1513: F, t4516: F) -> (F, F, F, F, F, F) {
    let t5761 = t1552 * t506 * t19;
    let t5763 = t799 * t299 * t481;
    let t5764 = t5761 * t5763;
    let t5767 = t119 * t155 * t1533;
    let t5768 = t1557 * t5767;
    let t5770 = t1513 * t5767;
    let t5771 = F::new(0.14615125e1) * t5770;
    let t5772 = param_hyb_omega_0 * t4516;
    (t5761, t5763, t5764, t5768, t5771, t5772)
}
