//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 459/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk459<F: Float>(t1482: F, t3841: F, t542: F, t1360: F, t3793: F, t3795: F, t3799: F, t3803: F, t3807: F, t469: F, t1311: F, t1315: F) -> (F, F, F, F, F, F, F) {
    let t3842 = t1482 * t3841;
    let t3843 = t542 * t3842;
    let t3846 = t1360 * t1360;
    let t3848 = F::new(0.23744444444444444444e-1) * t3793;
    let t3853 = t3848 + F::new(0.11872222222222222222e-1) * t3795 - F::new(0.11872222222222222222e-1) * t3799 + F::new(0.35616666666666666666e-1) * t3803 - F::new(0.17808333333333333333e-1) * t3807;
    let t3855 = F::new(0.62182e-1) * t3853 * t469;
    let t3856 = t1311 * t1315;
    (t3842, t3843, t3846, t3848, t3853, t3855, t3856)
}
