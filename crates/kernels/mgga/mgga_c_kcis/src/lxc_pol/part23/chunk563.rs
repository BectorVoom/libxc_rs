//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 563/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk563<F: Float>(t5439: F, t5441: F, t1471: F, t1472: F, t167: F, t1098: F, t1992: F, t1102: F, t3743: F, t3744: F, t3746: F, t3748: F, t4587: F, t5423: F, t5428: F, t5432: F, t5436: F) -> (F, F, F) {
    let t5442 = t5439 * t5441;
    let t5446 = t1471 * t1472 * t167;
    let t5449 = t1098 * t1992;
    let t5451 = -t3743 + F::new(0.43802864444444444445e-3) * t3744 + F::new(0.98556445e-3) * t3746 - F::new(0.65704296666666666667e-3) * t3748 + F::new(0.43802864444444444445e-3) * t5423 + F::new(0.10950716111111111111e-2) * t1102 * t5428 + F::new(0.98556445e-3) * t1102 * t5432 - F::new(0.65704296666666666667e-3) * t1102 * t5436 - F::new(0.13140859333333333333e-2) * t1102 * t5442 - F::new(0.13140859333333333333e-2) * t4587 * t5446 + F::new(0.98556445e-3) * t5449;
    (t5442, t5446, t5451)
}
