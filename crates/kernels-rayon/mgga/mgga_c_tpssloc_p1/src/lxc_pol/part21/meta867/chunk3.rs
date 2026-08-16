//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3167/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3167(t11818: f64, t248: f64, t3506: f64, t6225: f64, t1174: f64, t11825: f64, t1214: f64, t1227: f64, t1230: f64, t15672: f64, t15761: f64, t1737: f64, t19026: f64, t19051: f64, t3440: f64, t3496: f64, t3511: f64, t3515: f64, t3518: f64, t3587: f64, t475: f64, t4889: f64, t5024: f64, t52568: f64, t6211: f64, t63311: f64, t63353: f64, t65264: f64, t65528: f64, t65541: f64, t65545: f64, t65552: f64, t65554: f64) -> f64 {
    let t65558 = t3506 * t248 * t11818 * t6225;
    let t65565 = -t1227 * t248 * t1230 * t63353 / 4608.0_f64 - t11825 * t6211 / 2304.0_f64 - t65528 / 13824.0_f64 + t52568 * t1737 / 1536.0_f64 - t3515 * t248 * t1214 * t65264 * t475 / 1536.0_f64 + 19.0_f64 / 1728.0_f64 * t19026 * t3496 + 19.0_f64 / 864.0_f64 * t65541 * t3511 - 19.0_f64 / 1728.0_f64 * t65545 * t3518 + t5024 * t15761 / 432.0_f64 + 5.0_f64 / 13824.0_f64 * t19051 * t3587 + t65552 / 10368.0_f64 + t65554 / 2304.0_f64 - t65558 / 6912.0_f64 - 4.0_f64 / 81.0_f64 * t4889 * t15672 + t1174 * t3440 * t63311 / 108.0_f64;
    t65565
}
