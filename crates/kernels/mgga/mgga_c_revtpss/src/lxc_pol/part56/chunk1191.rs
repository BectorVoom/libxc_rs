//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1191/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1191<F: Float>(t1243: F, t1769: F, t1774: F, t34925: F, t73: F, t124668: F, t1248: F, t124869: F, t124994: F, t124996: F, t125003: F, t125009: F, t125012: F, t125028: F, t1287: F, t1294: F, t2142: F, t2148: F, t29109: F, t29158: F, t29159: F, t3153: F, t33461: F, t33462: F, t33477: F, t33478: F, t34914: F, t34915: F, t5480: F, t7627: F, t8190: F, t8217: F) -> (F, F) {
    let t131934 = t1243 * t1769;
    let t131939 = t1243 * t1774;
    let t131962 = t34925 * t73;
    let t131966 = F::cast_from(0.37187329209051010821e-3_f64) * t124994 - F::cast_from(0.17347256376410398924e1_f64) * t2148 * t7627 * t8217 + F::cast_from(0.8673628188205199462e0_f64) * t124869 * t34925 * t3153 * t5480 - F::cast_from(0.17347256376410398924e1_f64) * t125003 * t131934 * t1248 * t1287 + F::cast_from(0.17347256376410398924e1_f64) * t124996 * t131939 * t1248 * t1287 - F::cast_from(0.51407763898592117355e1_f64) * t33461 * t33478 * t34914 * t1294 + F::cast_from(0.11423947533020470523e1_f64) * t33477 * t33462 * t2142 * t29109 + F::cast_from(0.11423947533020470523e1_f64) * t33477 * t33462 * t7627 * t8190 + F::cast_from(0.22847895066040941046e1_f64) * t125009 * t29158 * t125012 + F::cast_from(0.37645955677973955998e-3_f64) * t125028 + F::cast_from(0.17135921299530705785e1_f64) * t124668 * t34915 - F::cast_from(0.17347256376410398924e1_f64) * t125003 * t131962 * t29159;
    (t131962, t131966)
}
