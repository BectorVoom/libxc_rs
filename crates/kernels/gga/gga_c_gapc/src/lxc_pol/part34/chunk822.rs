//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 822/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk822<F: Float>(t10136: F, t10170: F, t10200: F, t10240: F, t10283: F, t10323: F, t10370: F, t10405: F, t8292: F, t8298: F, t8301: F, t8304: F, t8306: F, t8311: F, t8314: F, t8317: F, t8319: F, t8322: F, t8324: F) -> (F, F) {
    let t10408 = t10136 + t10170 + t10200 + t10240 + t10283 + t10323 + t10370 + t10405;
    let t10433 = -0.59049582388386525904e-5 * t8292 - 0.59049582388386525904e-5 * t8298 - 0.86898242813537603826e-4 * t8301 + 0.43449121406768801913e-4 * t8304 + 0.20855578275249024918e-2 * t8306 + 0.12147342662753799615e-3 * t8311 + 0.86898242813537603826e-5 * t8314 + 0.86898242813537603826e-5 * t8317 + 0.41711156550498049836e-2 * t8319 - 0.82402707051983925121e-5 * t8322 - 0.60828769969476322678e-4 * t8324;
    (t10408, t10433)
}
