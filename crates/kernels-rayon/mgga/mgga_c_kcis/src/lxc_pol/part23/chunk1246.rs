//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1246/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1246(t28513: f64, t4142: f64, t1464: f64, t15956: f64, t28503: f64, t1394: f64, t5667: f64, t94216: f64, t16773: f64, t27387: f64, t5780: f64, t27364: f64, t4153: f64, t5663: f64) -> (f64, f64, f64, f64, f64) {
    let t98344 = t4142 * t28513;
    let t98347 = t1464 * t28503 * t15956;
    let t98350 = t1394 * t94216 * t5667;
    let t98353 = t5780 * t27387 * t16773;
    let t98357 = t4153 * t27364 * t5663;
    (t98344, t98347, t98350, t98353, t98357)
}
