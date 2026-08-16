//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 1274/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk1274<F: Float>(t190: F, t23608: F, t24110: F, t35729: F, t3643: F, t760: F, t10350: F, t11678: F, t11679: F, t24202: F, t11656: F, t11658: F, t24181: F) -> (F, F, F, F, F) {
    let t35732 = t35729 * t23608 * t190 * t24110;
    let t35734 = t3643 * t760;
    let t35736 = t35734 * t11678 * t10350;
    let t35738 = t11679 * t24202;
    let t35741 = t24181 * t11656 * t11658;
    (t35732, t35734, t35736, t35738, t35741)
}
