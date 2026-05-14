//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1330/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1330<F: Float>(t1333: F, t34806: F, t2213: F, t415: F, t5981: F, t1163: F, t26998: F, t6183: F, t114720: F, t2232: F, t1406: F, t7832: F, t26833: F, t468: F, t8082: F, t34826: F) -> (F, F, F, F, F, F, F, F) {
    let t119290 = t1333 * t34806;
    let t119293 = t415 * t2213 * t5981;
    let t119298 = t6183 * t26998 * t1163;
    let t119302 = t415 * t114720 * t2232;
    let t119305 = t415 * t1406 * t7832;
    let t119308 = t415 * t468 * t26833;
    let t119311 = t415 * t1406 * t8082;
    let t119313 = t1333 * t34826;
    (t119290, t119293, t119298, t119302, t119305, t119308, t119311, t119313)
}
