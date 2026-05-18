//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 695/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk695<F: Float>(t24886: F, t4266: F, t1495: F, t2766: F, t4141: F, t11593: F, t1901: F, t24882: F, t24884: F, t29052: F, t29057: F, t29060: F, t29064: F, t29068: F, t29073: F, t29077: F, t29084: F, t29087: F) -> F {
    let t29090 = t24886 * t4266;
    let t29093 = t2766 * t1495;
    let t29094 = t29093 * t4141;
    let t29097 = -F::new(2.0) / F::new(3.0) * t1901 * t29052 - F::new(2.0) / F::new(3.0) * t1901 * t29057 + t1901 * t29060 / F::new(9.0) + t1901 * t29064 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t11593 * t29068 - F::new(2.0) * t1901 * t29073 - F::new(2.0) / F::new(3.0) * t1901 * t29077 - t24882 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t24884 + t1901 * t29084 / F::new(9.0) + t1901 * t29087 / F::new(9.0) + F::new(2.0) / F::new(9.0) * t1901 * t29090 - F::new(2.0) / F::new(27.0) * t1901 * t29094;
    t29097
}
