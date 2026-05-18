//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1292/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1292<F: Float>(t11181: F, t11236: F, t4015: F, t11235: F, t14940: F, t1603: F, t8286: F, t11242: F, t203: F, t2922: F, t8296: F, t25514: F, t2920: F, t2974: F, t8290: F) -> (F, F, F, F) {
    let t35435 = t11181 * t4015 * t11236;
    let t35439 = t8286 * t14940 * t11235 * t1603;
    let t35443 = t2922 * t11242 * t203 * t8296;
    let t35447 = t2920 * t25514 * t2974 * t8290;
    (t35435, t35439, t35443, t35447)
}
