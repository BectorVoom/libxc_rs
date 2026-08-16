//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 957/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk957<F: Float>(t1701: F, t1702: F, t8932: F, t2035: F, t538: F, t8807: F, t554: F, t6: F, t8908: F, t133: F, t8909: F, t542: F, t7334: F) -> (F, F, F, F, F, F, F) {
    let t39835 = t1701 * t1702 * t8932;
    let t39839 = t2035 * t8807 * t538;
    let t39843 = t2035 * t8807 * t554;
    let t39846 = t8908 * t6;
    let t39847 = t133 * t39846;
    let t39849 = t1701 * t1702 * t8909;
    let t39852 = t542 * t7334;
    (t39835, t39839, t39843, t39846, t39847, t39849, t39852)
}
