//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 904/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk904<F: Float>(t13477: F, t3937: F, t1299: F, t389: F, t3934: F, t1319: F, t4065: F, t3938: F, t3935: F, t12983: F, t6175: F, t1293: F, t394: F) -> (F, F, F, F, F) {
    let t13478 = t3937 * t13477;
    let t13482 = t389 * t1299 * t3934;
    let t13485 = t4065 * t1319;
    let t13486 = t13485 * t3938;
    let t13487 = t3935 * t13486;
    let t13489 = t6175 * t12983;
    let t13493 = t1293 * t394 * t3934;
    (t13478, t13482, t13487, t13489, t13493)
}
