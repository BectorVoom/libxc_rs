//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1190/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1190<F: Float>(t12915: F, t247: F, t34964: F, t8926: F, t1243: F, t1828: F, t105460: F, t1203: F, t1214: F, t124605: F, t124659: F, t124694: F, t124706: F, t124772: F, t1248: F, t124903: F, t125017: F, t1287: F, t29187: F, t33449: F, t33462: F, t33469: F, t33484: F, t33485: F, t34914: F, t34949: F, t34960: F, t473: F, t5284: F, t5497: F, t7627: F, t7637: F, t8201: F, t8932: F) -> F {
    let t131907 = t8926 * t247 * t12915 * t34964;
    let t131920 = t1243 * t1828;
    let t131925 = -F::cast_from(0.3427184259906141157e1_f64) * t33469 * t33462 * t8201 * t7627 - F::cast_from(0.34694512752820797848e1_f64) * t124694 * t29187 + F::cast_from(0.11423947533020470523e1_f64) * t124903 * t34949 + F::cast_from(0.11423947533020470523e1_f64) * t33484 * t33485 * t5284 * t1287 + F::cast_from(0.3427184259906141157e1_f64) * t124659 * t33462 * t34914 * t1203 - F::cast_from(0.51407763898592117355e1_f64) * t124706 * t33462 * t34914 * t1214 - F::cast_from(0.18822977838986977999e-3_f64) * t131907 + F::cast_from(0.34694512752820797848e1_f64) * t124605 * t7637 * t105460 - F::cast_from(0.3427184259906141157e1_f64) * t125017 * t34960 * t1248 * t1287 + F::cast_from(0.17347256376410398924e1_f64) * t8932 * t33449 * t473 * t5497 + F::cast_from(0.17347256376410398924e1_f64) * t124772 * t131920 * t1248 * t1287;
    t131925
}
