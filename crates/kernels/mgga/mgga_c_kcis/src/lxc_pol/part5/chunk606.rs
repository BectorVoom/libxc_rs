//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 606/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk606<F: Float>(t468: F, t3793: F, t1341: F, t45: F, t1346: F, t478: F) -> (F, F, F, F, F) {
    let t3900 = t468 * t468;
    let t3901 = F::new(1.0) / t3900;
    let t3905 = F::new(0.12361111111111111111e-1) * t3793;
    let t3914 = t45 * t1341;
    let t3917 = t1346 * t478;
    let t3918 = F::new(1.0) / t3917;
    (t3900, t3901, t3905, t3914, t3918)
}
