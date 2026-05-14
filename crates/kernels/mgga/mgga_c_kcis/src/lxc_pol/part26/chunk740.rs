//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 740/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk740<F: Float>(t5: F, t66: F, t728: F, t119: F, t122: F, t144: F, t145: F, t2552: F, t2559: F, t2572: F, t784: F, t788: F, t85: F, t9070: F, t9074: F, t9078: F, t9082: F, t9089: F, t9092: F, t9094: F, t9097: F, t9099: F, t9103: F, t9105: F) -> (F,) {
    let t9109 = t5 * t66 * t728;
    let t9112 = -0.1857375e-1 * t784 * t2572 + 0.619125e-2 * t9070 * t145 - 0.8255e-2 * t2552 * t9074 + 0.371475e-1 * t2559 * t9078 - 0.38523333333333333333e-1 * t788 * t9082 - 0.23583209876543209876e-1 * t85 * t119 * t122 - 0.371475e-1 * t144 * t9089 + 0.371475e-1 * t9092 * t9094 + 0.41275e-2 * t9097 * t9099 - 0.74295e-1 * t9103 * t9105 - 0.4953e-1 * t2559 * t9109;
    (t9112,)
}
