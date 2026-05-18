//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1181/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1181<F: Float>(t1218: F, t1253: F, t143538: F, t1466: F, t154242: F, t154492: F, t154794: F, t154833: F, t193: F, t24964: F, t28955: F, t28985: F, t28993: F, t33808: F, t34251: F, t34260: F, t35802: F, t36011: F, t4027: F, t6210: F, t7581: F, t7684: F) -> F {
    let t155092 = -F::new(4.0) * t154833 + t143538 / F::new(9.0) - t4027 * t7684 - F::new(4.0) * t154242 - F::new(2.0) * t154492 - F::new(2.0) / F::new(3.0) * t1466 * t193 * t24964 * t36011 - F::new(2.0) * t154794 - t1218 * t34251 + t7581 * t28955 / F::new(6.0) - t33808 * t28993 / F::new(18.0) - F::new(2.0) / F::new(3.0) * t6210 * t35802 - F::new(2.0) / F::new(3.0) * t1466 * t193 * t24964 * t28985 + t1466 * t193 * t34260 * t1253 / F::new(6.0);
    t155092
}
