//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 506/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk506<F: Float>(t2953: F, t2954: F, t1005: F, t1599: F, t1603: F, t2937: F, t1027: F, t659: F, t126: F, t615: F) -> (F, F, F, F, F, F) {
    let t2955 = t2953 * t2954;
    let t2957 = t1005 * t1599;
    let t2958 = t2937 * t1603;
    let t2959 = t2957 * t2958;
    let t2970 = t1027 * t659;
    let t2972 = t126 * t615;
    (t2955, t2957, t2958, t2959, t2970, t2972)
}
