//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 501/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk501<F: Float>(t3821: F, t676: F, t27: F, t89: F, t2335: F, t2338: F, t2341: F, t3688: F, t3693: F, t3697: F, t3702: F, t3707: F, t3710: F, t3715: F, t3720: F) -> (F, F, F) {
    let t3822 = t676 * t3821;
    let t3824 = t89 * t27 * t3822;
    let t3826 = t2335 + t2338 / F::new(54.0) + t2341 / F::new(18.0) + t3688 / F::new(54.0) - t3693 / F::new(27.0) + t3697 / F::new(18.0) + t3702 / F::new(9.0) + t3707 / F::new(9.0) + t3710 / F::new(18.0) + t3715 / F::new(18.0) + t3720 / F::new(3.0) - t3824 / F::new(6.0);
    (t3822, t3824, t3826)
}
