//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 390/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk390<F: Float>(t2179: F, t5956: F, t144: F, t5897: F, t5914: F, t5894: F, t5903: F, t5907: F, t5911: F, t5919: F, t5923: F, t5927: F) -> (F, F, F, F) {
    let t5957 = t2179 * t5956;
    let t5958 = t144 * t5957;
    let t5962 = t5897 / 6.0;
    let t5965 = t5914 / 3.0;
    let t5968 = t5894 / 4.0 + t5962 + t5903 / 6.0 + t5907 - t5911 / 2.0 + t5965 + t5919 / 3.0 + 2.0 * t5923 - t5927;
    (t5958, t5962, t5965, t5968)
}
