//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 968/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk968<F: Float>(t2132: F, t322: F, t7896: F, t7997: F, t2133: F, t879: F, t3915: F, t7948: F, t1221: F, t2131: F, t8004: F, t2147: F, t463: F, t7885: F, t7886: F) -> (F, F, F, F, F) {
    let t32048 = F::new(0.52041769129231196772e1) * t7896 * t2132 * t7997 * t322;
    let t32052 = F::new(0.52041769129231196772e1) * t7896 * t2132 * t2133 * t879;
    let t32054 = F::new(0.39512695097613069591e1) * t7948 * t3915;
    let t32057 = t2131 * t8004 * t2133 * t1221;
    let t32061 = t7885 * t2147 * t7886 * t463;
    (t32048, t32052, t32054, t32057, t32061)
}
