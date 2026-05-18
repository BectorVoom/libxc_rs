//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 846/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk846<F: Float>(t10658: F, t14949: F, t19838: F, t19839: F, t19852: F, t19857: F, t19858: F, t19859: F, t21981: F, t22164: F, t22332: F, t22336: F, t22339: F) -> F {
    let t22438 = -F::new(2.0) * t21981 - t14949 + t19838 - t19839 - t22164 / F::new(3.0) + t22332 / F::new(6.0) + t22336 / F::new(8.0) - t22339 / F::new(4.0) - t19852 - t10658 + t19857 - t19858 + t19859;
    t22438
}
