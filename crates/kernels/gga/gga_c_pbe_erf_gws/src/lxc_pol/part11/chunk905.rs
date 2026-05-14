//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 905/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk905<F: Float>(t12517: F, t184: F, t202: F, t12501: F, t639: F, t7877: F, t12869: F, t1791: F, t1006: F, t10871: F, t10969: F, t2796: F, t1022: F, t1031: F, t12514: F, t395: F) -> (F, F, F, F, F, F, F) {
    let t40790 = t202 * t12517 * t184;
    let t40824 = t639 * t7877 * t12501;
    let t40855 = t1791 * t12869;
    let t40865 = t1006 * t10871;
    let t40867 = t10969 * t2796;
    let t40899 = t1022 * t1031 * t184;
    let t40954 = t395 * t12514;
    (t40790, t40824, t40855, t40865, t40867, t40899, t40954)
}
