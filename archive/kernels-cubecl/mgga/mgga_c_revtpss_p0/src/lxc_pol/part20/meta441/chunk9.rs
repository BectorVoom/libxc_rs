//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1684/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1684<F: Float>(t12587: F, t3794: F, t3798: F, t45282: F, t45296: F, t45298: F, t45302: F, t45306: F, t45310: F, t45312: F, t45316: F, t45318: F, t45321: F, t45323: F, t45326: F, t5023: F) -> F {
    let t45908 = F::cast_from(12.0_f64) * t12587 * t3794 * t3798 * t5023 - t45282 + t45296 - t45298 - t45302 + t45306 - t45310 + t45312 - t45316 - t45318 - t45321 - t45323 + t45326;
    t45908
}
