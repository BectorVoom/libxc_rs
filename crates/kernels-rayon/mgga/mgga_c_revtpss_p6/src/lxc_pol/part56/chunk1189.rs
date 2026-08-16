//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1189/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1189(t12916: f64, t33414: f64, t34956: f64, t33508: f64, t34969: f64, t1042: f64, t105236: f64, t124564: f64, t124665: f64, t124887: f64, t124928: f64, t124942: f64, t124964: f64, t12787: f64, t1795: f64, t21028: f64, t29283: f64, t33425: f64, t33461: f64, t33462: f64, t33505: f64, t33512: f64, t3362: f64, t34909: f64, t34945: f64, t34982: f64, t3555: f64, t4181: f64, t494: f64, t5215: f64, t7652: f64, t8931: f64) -> f64 {
    let t131861 = t33414 * t12916 * t34956;
    let t131863 = t34969 * t33508;
    let t131882 = 0.34694512752820797848e1_f64 * t124887 * t7652 * t105236 + 0.11423947533020470523e1_f64 * t124928 * t34909 + 0.31371629731644963332e-3_f64 * t33425 * t12787 * t494 * t3362 * t4181 + 0.18822977838986977999e-3_f64 * t131861 + t124942 + 0.3718732920905101082e-3_f64 * t131863 * t33512 - 0.17347256376410398924e1_f64 * t3555 * t8931 * t34982 + 0.3718732920905101082e-3_f64 * t124564 * t1042 * t1795 * t21028 - 0.18822977838986977999e-3_f64 * t124964 + 0.34694512752820797848e1_f64 * t124665 * t29283 + 0.17135921299530705785e1_f64 * t33461 * t33462 * t8931 * t5215 + 0.99166211224136028853e-3_f64 * t33505 * t34945;
    t131882
}
