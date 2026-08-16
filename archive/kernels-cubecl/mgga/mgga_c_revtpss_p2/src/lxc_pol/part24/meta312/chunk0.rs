//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1099/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1099<F: Float>(t6800: F, t749: F, t512: F, t177: F, t762: F, t1877: F, t73: F, t4010: F, t6836: F, t1412: F, t6816: F, t221: F, t4019: F, t6844: F) -> (F, F, F, F, F, F, F, F) {
    let t22195 = t6800 * t749;
    let t22196 = t512 * t22195;
    let t22212 = t6800 * t177;
    let t22213 = t22212 * t762;
    let t22229 = t1877 * t73;
    let t22236 = t4010 * t6836;
    let t22245 = t1412 * t6816;
    let t22259 = t4019 * t221 * t6844;
    (t22195, t22196, t22212, t22213, t22229, t22236, t22245, t22259)
}
