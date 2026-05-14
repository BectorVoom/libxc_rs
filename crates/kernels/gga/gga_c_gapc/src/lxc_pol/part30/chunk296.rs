//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 296/1135 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk296<F: Float>(t1092: F, t1093: F, t1002: F, t1008: F, t992: F, t1014: F, t1020: F, t1024: F, t1028: F, t1041: F, t1047: F) -> (F, F, F) {
    let t1094 = t1092 * t1093;
    let t1104 = 0.20855578275249024918e-2 * t992 + 0.60736713313768998073e-4 * t1002 - 0.43449121406768801913e-4 * t1008;
    let t1112 = 0.27801896084645508334e-2 * t1014 + 0.20241536458333333335e-4 * t1020 - 0.17376185052903442709e-3 * t1024 - 0.2318836277704281739e-4 * t1028 - 0.16882592796244404291e-6 * t1041 + 0.14492726735651760868e-5 * t1047;
    (t1094, t1104, t1112)
}
