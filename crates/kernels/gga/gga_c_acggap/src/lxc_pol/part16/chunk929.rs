//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 929/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk929<F: Float>(t2132: F, t2133: F, t7896: F, t879: F, t3915: F, t7948: F, t2147: F, t463: F, t7885: F, t7886: F, t1960: F, t3912: F) -> (F, F, F, F) {
    let t32052 = F::new(0.52041769129231196772e1) * t7896 * t2132 * t2133 * t879;
    let t32054 = F::new(0.39512695097613069591e1) * t7948 * t3915;
    let t32061 = t7885 * t2147 * t7886 * t463;
    let t32073 = F::new(0.65854491829355115987e0) * t1960 * t3912;
    (t32052, t32054, t32061, t32073)
}
