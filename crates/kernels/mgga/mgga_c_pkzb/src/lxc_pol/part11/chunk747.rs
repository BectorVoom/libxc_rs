//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 747/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk747<F: Float>(t5955: F, t6012: F, t2019: F, t785: F, t306: F, t5718: F, t2030: F, t2036: F, t5931: F, t287: F, t2155: F, t314: F) -> (F, F, F, F, F, F, F, F) {
    let t6013 = t6012 * t5955;
    let t6017 = t2019 * t785;
    let t6026 = t5718 * t306;
    let t6027 = t6012 * t2030;
    let t6036 = t2036 * t785;
    let t6047 = t5931 * t306;
    let t6048 = t6012 * t287;
    let t6065 = F::new(1.0) / t2155 / t314;
    (t6013, t6017, t6026, t6027, t6036, t6047, t6048, t6065)
}
