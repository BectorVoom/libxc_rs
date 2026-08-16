//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 986/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk986<F: Float>(t3784: F, t9865: F, t11933: F, t19: F, t311: F, t3752: F, t3750: F, t869: F, t1453: F, t3760: F, t9555: F, t190: F, t6851: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11951 = t3784 * t9865;
    let t11953 = t11933 * t19;
    let t11954 = t311 * t11953;
    let t11955 = t11954 * t3752;
    let t11957 = t869 * t3750;
    let t11958 = t11957 * t3752;
    let t11960 = t3760 * t1453;
    let t11961 = t311 * t11960;
    let t11962 = t11961 * t9555;
    let t11964 = t6851 * t190;
    (t11951, t11953, t11954, t11955, t11957, t11958, t11960, t11961, t11962, t11964)
}
