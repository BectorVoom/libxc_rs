//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1030/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1030<F: Float>(t26519: F, t26653: F, t180: F, t7671: F, t838: F, t109: F, t209: F, t2167: F, t2651: F, t3707: F, t637: F, t2233: F) -> (F, F, F, F, F, F, F) {
    let t26654 = t26519 + t26653;
    let t26655 = t180 * t26654;
    let t26656 = t838 * t7671;
    let t26971 = t209 * t109;
    let t27154 = t2651 * t2167;
    let t27325 = t3707 * t637;
    let t27326 = t2233 * t27325;
    (t26654, t26655, t26656, t26971, t27154, t27325, t27326)
}
