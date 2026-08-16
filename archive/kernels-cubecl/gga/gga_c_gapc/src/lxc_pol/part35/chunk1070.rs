//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 1070/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk1070<F: Float>(t11798: F, t28370: F, t7453: F, t19048: F, t3284: F, t1736: F, t435: F, t1084: F, t3375: F, t11512: F, t3707: F, t7375: F) -> (F, F, F, F, F, F, F) {
    let t33298 = t11798 * t28370 * t7453;
    let t33301 = t11798 * t3284 * t19048;
    let t33303 = t435 * t1736;
    let t33304 = t1084 * t33303;
    let t33305 = t33304 * t3375;
    let t33307 = t11512 * t3707;
    let t33309 = t1084 * t33307 * t7375;
    (t33298, t33301, t33303, t33304, t33305, t33307, t33309)
}
