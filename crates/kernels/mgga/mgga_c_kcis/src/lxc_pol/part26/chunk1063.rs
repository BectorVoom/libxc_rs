//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1063/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1063<F: Float>(t26519: F, t26653: F, t180: F, t7671: F, t838: F, t109: F, t209: F, t4121: F, t541: F, t1014: F, t7932: F, t7935: F) -> (F, F, F, F, F, F, F) {
    let t26654 = t26519 + t26653;
    let t26655 = t180 * t26654;
    let t26656 = t838 * t7671;
    let t26657 = F::new(2.0) * t26656;
    let t26971 = t209 * t109;
    let t27331 = t541 * t4121;
    let t27335 = t1014 * t7932;
    let t27337 = t1014 * t7935;
    (t26654, t26655, t26657, t26971, t27331, t27335, t27337)
}
