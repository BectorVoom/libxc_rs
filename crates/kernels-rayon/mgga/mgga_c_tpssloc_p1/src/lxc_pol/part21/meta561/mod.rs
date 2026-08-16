//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta561 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2266;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2267;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta561(t1196: f64, t16558: f64, t974: f64, t1215: f64, t1653: f64, t15659: f64, t3578: f64, t1177: f64, t18221: f64, t18237: f64, t1735: f64, t4724: f64, t11668: f64, t18232: f64, t3440: f64, t1017: f64, t6163: f64, t1210: f64, t1207: f64, t11665: f64, t11678: f64, t1174: f64, t11834: f64, t1218: f64, t15569: f64, t15717: f64, t15719: f64, t15722: f64, t15740: f64, t3577: f64, t4889: f64, t4950: f64, t4954: f64, t4969: f64, t5046: f64, t6192: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18996, t18997, t19000, t19001, t19002, t19005, t19010, t19015) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2266(t1196, t16558, t974, t1215, t1653, t15659, t3578, t1177, t18221, t18237, t1735, t4724);
        let (t19016, t19025, t19026, t19029) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2267(t11668, t19015, t18232, t3440, t1017, t6163, t1210, t1207, t11665, t11678, t1174, t11834, t1218, t15569, t15717, t15719, t15722, t15740, t18997, t19002, t19005, t19010, t3577, t4889, t4950, t4954, t4969, t5046, t6192);
    (t18996, t19000, t19001, t19002, t19015, t19016, t19025, t19026, t19029)
}
