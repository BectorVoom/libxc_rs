//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 944/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk944<F: Float>(t1989: F, t678: F, t17531: F, t17533: F, t17536: F, t17539: F, t17543: F, t17545: F, t17548: F, t17549: F, t17552: F, t17553: F) -> F {
    let t17555 = t1989 * t678;
    let t17557 = t17531 + t17533 - t17536 - t17539 + t17543 + t17545 + t17548 + F::new(16.0) * t17549 + t17552 + F::new(16.0) / F::new(3.0) * t17553 + F::new(16.0) * t17555;
    t17557
}
