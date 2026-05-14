//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 674/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk674<F: Float>(t5761: F, t5763: F, t119: F, t1533: F, t155: F, t1557: F, t1513: F, t4516: F, t103: F, t2: F, t39: F, t102: F, t120: F, t5645: F, t506: F, t497: F, t542: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5764 = t5761 * t5763;
    let t5767 = t119 * t155 * t1533;
    let t5768 = t1557 * t5767;
    let t5770 = t1513 * t5767;
    let t5771 = 0.14615125e1 * t5770;
    let t5772 = param_hyb_omega_0 * t4516;
    let t5773 = t103 * t2;
    let t5776 = 0.19486833333333333333e1 * t5772 * t5773 * t39;
    let t5779 = 0.2923025e1 * t102 * t120 * t5645;
    let t5780 = t506 * t5645;
    let t5783 = t542 * t497;
    (t5764, t5768, t5771, t5772, t5773, t5776, t5779, t5780, t5783)
}
