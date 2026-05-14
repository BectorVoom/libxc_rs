//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1037/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1037<F: Float>(t12916: F, t33414: F, t34956: F, t33508: F, t34969: F, t1042: F, t105236: F, t124564: F, t124665: F, t124887: F, t124928: F, t124942: F, t124964: F, t12787: F, t1795: F, t21028: F, t29283: F, t33425: F, t33461: F, t33462: F, t33505: F, t33512: F, t3362: F, t34909: F, t34945: F, t34982: F, t3555: F, t4181: F, t494: F, t5215: F, t7652: F, t8931: F) -> (F,) {
    let t131861 = t33414 * t12916 * t34956;
    let t131863 = t34969 * t33508;
    let t131882 = 0.34694512752820797848e1 * t124887 * t7652 * t105236 + 0.11423947533020470523e1 * t124928 * t34909 + 0.31371629731644963332e-3 * t33425 * t12787 * t494 * t3362 * t4181 + 0.18822977838986977999e-3 * t131861 + t124942 + 0.3718732920905101082e-3 * t131863 * t33512 - 0.17347256376410398924e1 * t3555 * t8931 * t34982 + 0.3718732920905101082e-3 * t124564 * t1042 * t1795 * t21028 - 0.18822977838986977999e-3 * t124964 + 0.34694512752820797848e1 * t124665 * t29283 + 0.17135921299530705785e1 * t33461 * t33462 * t8931 * t5215 + 0.99166211224136028853e-3 * t33505 * t34945;
    (t131882,)
}
