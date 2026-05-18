//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 773/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk773<F: Float>(t1114: F, t4670: F, t345: F, t2918: F, t2919: F, t4612: F, t4615: F, t4618: F, t4623: F, t261: F, t1666: F, t930: F) -> (F, F, F, F, F) {
    let t4671 = t1114 * t4670;
    let t4672 = t345 * t4671;
    let t4682 = t2918 + F::new(0.5936111111111111111e-2) * t2919 + F::new(0.5936111111111111111e-2) * t4612 - F::new(0.11872222222222222222e-1) * t4615 + F::new(0.35616666666666666666e-1) * t4618 - F::new(0.35616666666666666666e-1) * t4623;
    let t4684 = F::new(0.62182e-1) * t4682 * t261;
    let t4685 = t1666 * t930;
    (t4671, t4672, t4682, t4684, t4685)
}
