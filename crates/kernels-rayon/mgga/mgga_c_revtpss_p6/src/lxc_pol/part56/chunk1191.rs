//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1191/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1191(t1243: f64, t1769: f64, t1774: f64, t34925: f64, t73: f64, t124668: f64, t1248: f64, t124869: f64, t124994: f64, t124996: f64, t125003: f64, t125009: f64, t125012: f64, t125028: f64, t1287: f64, t1294: f64, t2142: f64, t2148: f64, t29109: f64, t29158: f64, t29159: f64, t3153: f64, t33461: f64, t33462: f64, t33477: f64, t33478: f64, t34914: f64, t34915: f64, t5480: f64, t7627: f64, t8190: f64, t8217: f64) -> (f64, f64) {
    let t131934 = t1243 * t1769;
    let t131939 = t1243 * t1774;
    let t131962 = t34925 * t73;
    let t131966 = 0.37187329209051010821e-3_f64 * t124994 - 0.17347256376410398924e1_f64 * t2148 * t7627 * t8217 + 0.8673628188205199462e0_f64 * t124869 * t34925 * t3153 * t5480 - 0.17347256376410398924e1_f64 * t125003 * t131934 * t1248 * t1287 + 0.17347256376410398924e1_f64 * t124996 * t131939 * t1248 * t1287 - 0.51407763898592117355e1_f64 * t33461 * t33478 * t34914 * t1294 + 0.11423947533020470523e1_f64 * t33477 * t33462 * t2142 * t29109 + 0.11423947533020470523e1_f64 * t33477 * t33462 * t7627 * t8190 + 0.22847895066040941046e1_f64 * t125009 * t29158 * t125012 + 0.37645955677973955998e-3_f64 * t125028 + 0.17135921299530705785e1_f64 * t124668 * t34915 - 0.17347256376410398924e1_f64 * t125003 * t131962 * t29159;
    (t131962, t131966)
}
