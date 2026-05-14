//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 785/1135 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk785<F: Float>(t10398: F, t3217: F, t1061: F, t6925: F, t3239: F, t6927: F, t8292: F, t8298: F, t8301: F, t8304: F, t8306: F, t8311: F, t8314: F, t8317: F, t8319: F, t8322: F, t8324: F) -> (F, F, F) {
    let t10399 = t3217 * t10398;
    let t10401 = t1061 * t6925;
    let t10402 = t3239 * t6927;
    let t10403 = t10401 * t10402;
    let t10433 = -0.59049582388386525904e-5 * t8292 - 0.59049582388386525904e-5 * t8298 - 0.86898242813537603826e-4 * t8301 + 0.43449121406768801913e-4 * t8304 + 0.20855578275249024918e-2 * t8306 + 0.12147342662753799615e-3 * t8311 + 0.86898242813537603826e-5 * t8314 + 0.86898242813537603826e-5 * t8317 + 0.41711156550498049836e-2 * t8319 - 0.82402707051983925121e-5 * t8322 - 0.60828769969476322678e-4 * t8324;
    (t10399, t10403, t10433)
}
