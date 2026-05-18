//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1143/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1143<F: Float>(t2466: F, t2472: F, t2476: F, t845: F, t10838: F, t4038: F, t8272: F, t1885: F, t1891: F) -> (F, F, F, F) {
    let t23817 = t2466 * t2466;
    let t23821 = F::new(0.51947267698127589897e2) * t845 * t2472 * t23817 * t2476;
    let t23823 = t4038 * t10838 * t8272;
    let t23825 = t1891 * t1885;
    (t23817, t23821, t23823, t23825)
}
