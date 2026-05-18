//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1020/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1020<F: Float>(t32135: F, t45963: F, t10301: F, t32148: F, t32141: F, t45972: F, t10309: F, t116: F, t32160: F, t25081: F, t8567: F, t11064: F, t8489: F) -> (F, F, F, F, F, F, F, F, F) {
    let t119465 = t45963 * t32135;
    let t119468 = t10301 * t32148;
    let t119471 = t10301 * t32135;
    let t119500 = t10301 * t32141;
    let t119503 = t45972 * t32135;
    let t119508 = t10309 * t32148;
    let t119535 = t32160 * t116;
    let t119578 = t8567 * t25081;
    let t119675 = t8489 * t11064;
    (t119465, t119468, t119471, t119500, t119503, t119508, t119535, t119578, t119675)
}
