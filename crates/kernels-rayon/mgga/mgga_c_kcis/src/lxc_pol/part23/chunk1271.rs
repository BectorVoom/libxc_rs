//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1271/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1271(t1464: f64, t27423: f64, t98470: f64, t17254: f64, t2243: f64, t303: f64, t2237: f64, t54162: f64, t8158: f64, t1394: f64, t15838: f64, t27387: f64) -> (f64, f64, f64, f64) {
    let t98754 = t1464 * t98470 * t27423;
    let t98767 = t303 * t17254 * t2243;
    let t98777 = t2237 * t54162 * t8158;
    let t98781 = t1394 * t27387 * t15838;
    (t98754, t98767, t98777, t98781)
}
