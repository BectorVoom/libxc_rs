//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2700/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2700(t16448: f64, t225: f64, t12020: f64, t1842: f64, t12023: f64, t12026: f64, t12030: f64, t1372: f64, t1375: f64, t1385: f64, t1386: f64, t16022: f64, t16030: f64, t16122: f64, t16436: f64, t16439: f64, t16475: f64, t26224: f64, t3882: f64, t3887: f64, t3889: f64, t3911: f64, t3912: f64, t5215: f64, t5326: f64, t5353: f64, t5354: f64, t568: f64) -> f64 {
    let t55093 = t16448 * t225;
    let t55118 = t12020 * t1842;
    let t55124 = 6.0_f64 * t1375 * t1385 * t16436 * t3887 + 6.0_f64 * t1375 * t3887 * t3911 * t5353 - 18.0_f64 * t12026 * t26224 * t55118 + 3.0_f64 * t1372 * t16122 * t568 - 6.0_f64 * t12023 * t5215 + 6.0_f64 * t12030 * t5326 - 3.0_f64 * t12030 * t5354 - 6.0_f64 * t1386 * t55093 - 3.0_f64 * t16022 * t3912 - 3.0_f64 * t16030 * t3912 + 6.0_f64 * t16439 * t3889 - 3.0_f64 * t16439 * t3912 - 18.0_f64 * t16475 * t3882;
    t55124
}
