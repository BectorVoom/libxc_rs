//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1162/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1162(t13551: f64, t16039: f64, t3: f64, t4637: f64, t645: f64, t3537: f64, t4555: f64, t116: f64, t4674: f64, t117: f64, t13546: f64, t1279: f64, t1281: f64, t1668: f64, t1670: f64, t4549: f64, t4556: f64, t4559: f64, t547: f64, t5470: f64, t5474: f64, t5477: f64, t548: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16040 = t13551 + t16039;
    let t16041 = t3 * t16040;
    let t16052 = param_d * t16040;
    let t16064 = t645 * t4637;
    let t16067 = t4555 * t3537;
    let t16072 = t116 * t4674;
    let t16073 = t16072 * t645;
    let t16076 = t117 * t13546;
    let t16079 = 6.0_f64 * t1279 * t5474 + 3.0_f64 * t1279 * t5477 + 3.0_f64 * t1281 * t5470 + t16052 * t548 + 6.0_f64 * t16064 * t547 + 12.0_f64 * t16067 * t547 + 6.0_f64 * t16073 * t547 + 3.0_f64 * t16076 * t547 + 12.0_f64 * t1668 * t4556 + 6.0_f64 * t1668 * t4559 + 6.0_f64 * t1670 * t4549;
    (t16041, t16052, t16064, t16067, t16073, t16076, t16079)
}
