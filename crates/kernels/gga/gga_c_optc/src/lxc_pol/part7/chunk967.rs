//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 967/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk967<F: Float>(t8557: F, t8567: F, t8571: F, t8576: F, t8579: F, t8585: F, t8682: F, t8691: F, t8742: F, t8901: F, t8903: F, t1214: F, t2905: F) -> (F, F) {
    let t9265 = -t8742 + t8901 - t8567 + t8571 + t8576 + t8579 - t8585 + t8682 + t8691 + t8903 - t8557;
    let t9266 = t2905 * t1214;
    (t9265, t9266)
}
