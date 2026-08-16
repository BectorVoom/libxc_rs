//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1237/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1237(t20228: f64, t20248: f64, t20269: f64, t20289: f64, t20309: f64, t20338: f64, t20689: f64, t20706: f64, t1281: f64, t6856: f64, t1291: f64, t6860: f64) -> (f64, f64, f64) {
    let t20709 = t20228 + t20248 + t20269 + t20289 + t20309 + t20338 + t20689 + t20706;
    let t20711 = t6856 * t1281;
    let t20721 = t6860 * t1291;
    (t20709, t20711, t20721)
}
