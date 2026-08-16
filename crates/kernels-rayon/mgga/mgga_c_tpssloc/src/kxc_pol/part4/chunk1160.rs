//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1160/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1160(t19095: f64, t3515: f64, t1230: f64, t18241: f64, t248: f64, t11546: f64, t18206: f64, t11738: f64, t1174: f64, t1218: f64, t1227: f64, t1232: f64, t15591: f64, t15594: f64, t15754: f64, t1737: f64, t1748: f64, t19077: f64, t19080: f64, t19083: f64, t19087: f64, t19090: f64, t3490: f64, t4889: f64, t5002: f64, t5005: f64, t5014: f64, t5030: f64, t5033: f64, t6207: f64, t6211: f64) -> f64 {
    let t19096 = t3515 * t19095;
    let t19101 = t248 * t1230 * t18241;
    let t19106 = t11546 * t18206;
    let t19117 = t11738 * t19077 / 3072.0_f64 - t19080 * t1218 / 288.0_f64 + t19083 * t1232 / 432.0_f64 + t15754 / 648.0_f64 - t1174 * t19087 / 72.0_f64 + 11.0_f64 / 324.0_f64 * t19090 - 2.0_f64 / 81.0_f64 * t4889 * t5033 - t19096 / 4608.0_f64 - t3490 * t6207 / 4608.0_f64 - t1227 * t19101 / 4608.0_f64 - t3490 * t6211 / 2304.0_f64 - 7.0_f64 / 648.0_f64 * t1174 * t19106 + t15591 * t1737 / 1536.0_f64 + t5002 * t5014 / 1536.0_f64 - t15594 * t1748 / 2304.0_f64 - t5005 * t5030 / 2304.0_f64;
    t19117
}
