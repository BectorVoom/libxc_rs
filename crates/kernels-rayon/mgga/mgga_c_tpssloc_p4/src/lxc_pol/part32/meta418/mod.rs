//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta418 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1616;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1617;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1618;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1619;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta418(t15453: f64, t17686: f64, t4582: f64, t17635: f64, t4972: f64, t1090: f64, t6230: f64, t3578: f64, t6219: f64, t4997: f64, t5002: f64, t11784: f64, t248: f64, t5971: f64, t1227: f64, t5019: f64, t4993: f64, t5005: f64, t1202: f64, t6164: f64, t5024: f64, t11692: f64, t11792: f64, t11821: f64, t15671: f64, t15691: f64, t15699: f64, t15740: f64, t3577: f64, t488: f64, t4950: f64, t1196: f64, t16558: f64, t974: f64, t1215: f64, t1653: f64, t15659: f64, t1177: f64, t18221: f64, t18237: f64, t1735: f64, t4724: f64, t11668: f64, t18232: f64, t3440: f64, t1017: f64, t6163: f64, t1210: f64, t1207: f64, t11665: f64, t11678: f64, t1174: f64, t11834: f64, t1218: f64, t15569: f64, t15717: f64, t15719: f64, t15722: f64, t4889: f64, t4954: f64, t4969: f64, t5046: f64, t6192: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18955, t18959, t18965, t18969, t18972, t18975) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1616(t15453, t17686, t4582, t17635, t4972, t1090, t6230, t3578, t6219, t4997, t5002, t11784, t248, t5971);
        let t18989 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1617(t1227, t18975, t4997, t5019, t4993, t5005, t1202, t6164, t5024, t11692, t11792, t11821, t15671, t15691, t15699, t15740, t18955, t18959, t18965, t18969, t18972, t3577, t488, t4950);
        let (t18997, t19002, t19005, t19010, t19015) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1618(t1196, t16558, t974, t1215, t1653, t15659, t3578, t1177, t18221, t18237, t1735, t4724);
        let (t19016, t19024, t19029) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1619(t11668, t19015, t18232, t3440, t1017, t6163, t1210, t1207, t11665, t11678, t1174, t11834, t1218, t15569, t15717, t15719, t15722, t15740, t18997, t19002, t19005, t19010, t3577, t4889, t4950, t4954, t4969, t5046, t6192);
    (t18955, t18959, t18965, t18969, t18975, t18989, t18997, t19002, t19016, t19024, t19029)
}
