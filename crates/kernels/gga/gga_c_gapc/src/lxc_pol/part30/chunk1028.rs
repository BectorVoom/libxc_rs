//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1028/1135 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1028<F: Float>(t11492: F, t34675: F, t11320: F, t11496: F, t628: F, t11499: F, t34372: F, t8621: F, t1908: F, t22117: F, t3699: F, t5144: F, t116: F, t1899: F, t33666: F, t11503: F, t8916: F) -> (F, F, F, F, F, F) {
    let t34804 = t34675 * t11492;
    let t34808 = t628 * t11320 * t11496;
    let t34811 = t628 * t11499 * t11496;
    let t34813 = t34372 * t8621;
    let t34819 = t3699 * t22117 * t1908 * t5144;
    let t34820 = t116 * t1899 * t33666 * t34819;
    let t34822 = t8916 * t11503;
    (t34804, t34808, t34811, t34813, t34820, t34822)
}
