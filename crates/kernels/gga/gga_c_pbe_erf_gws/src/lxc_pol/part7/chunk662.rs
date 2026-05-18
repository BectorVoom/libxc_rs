//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 662/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk662<F: Float>(t4952: F, t5264: F, t4976: F, t606: F, t4939: F, t25: F, t4960: F, t4965: F, t5248: F, t5250: F, t5253: F, t5256: F, t5258: F, t5260: F) -> (F, F, F) {
    let t5265 = t5264 * t4952;
    let t5268 = t606 * t4976;
    let t5271 = F::new(0.11197407407407407407e0) * t4939;
    let t5272 = F::new(0.14396666666666666667e0) * t4960 - F::new(0.71983333333333333335e-1) * t4965 - F::new(0.26666666666666666667e-1) * t5248 + F::new(0.13333333333333333333e-1) * t25 * t5250 - F::new(0.66666666666666666666e-2) * t25 * t5253 - F::new(0.22222222222222222222e-1) * t5256 + F::new(0.13333333333333333334e-1) * t5258 + F::new(0.44444444444444444445e-2) * t5260 - F::new(0.29629629629629629629e-2) * t25 * t5265 - F::new(0.66666666666666666667e-2) * t25 * t5268 - t5271;
    (t5265, t5268, t5272)
}
