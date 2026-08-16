//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1013/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1013<F: Float>(t7799: F, t8545: F, t8491: F, t336: F, t4838: F, t578: F, t599: F, t30402: F, t31309: F, t525: F, t7325: F, t30371: F, t5152: F) -> (F, F, F, F, F) {
    let t34056 = t7799 * t8545;
    let t34059 = t7799 * t8491;
    let t34063 = t578 * t336 * t599 * t4838;
    let t34068 = t31309 * t30402 * t7325 * t525;
    let t34072 = t30371 * t5152;
    (t34056, t34059, t34063, t34068, t34072)
}
