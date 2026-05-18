//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 942/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk942<F: Float>(t1872: F, t5379: F, t1879: F, t5335: F, t1672: F, t1734: F, t616: F, t185: F, t5178: F, t582: F, t172: F, t184: F, t4980: F) -> (F, F, F, F, F) {
    let t17531 = F::new(8.0) / F::new(5.0) * t5379 * t1872;
    let t17533 = F::new(16.0) / F::new(15.0) * t1879 * t5335;
    let t17535 = t616 * t1672 * t1734;
    let t17536 = F::new(16.0) / F::new(45.0) * t17535;
    let t17538 = t185 * t582 * t5178;
    let t17539 = F::new(32.0) / F::new(15.0) * t17538;
    let t17541 = t172 * t4980 * t184;
    (t17531, t17533, t17536, t17539, t17541)
}
