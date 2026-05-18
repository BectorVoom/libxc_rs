//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1167/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1167<F: Float>(t10232: F, t29860: F, t29862: F, t29865: F, t29868: F, t29871: F, t29876: F, t29879: F, t29892: F, t31565: F, t31568: F, t31570: F, t31575: F, t31577: F, t4141: F) -> F {
    let t31578 = t29860 - t29862 - t29865 - t29868 + t29871 + t29876 - t29879 - t31565 - t31568 + t29892 + t31570 - F::new(0.31616674039640166222e-2) * t4141 * t10232 + t31575 + t31577;
    t31578
}
