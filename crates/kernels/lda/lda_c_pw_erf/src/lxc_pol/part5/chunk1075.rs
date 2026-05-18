//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1075/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1075<F: Float>(t8464: F, t8482: F, t8486: F, t14435: F, t11250: F, t11254: F, t11256: F, t14432: F, t14433: F, t14437: F, t14439: F, t8469: F, t8473: F, t8477: F, t8481: F, t8491: F, t8505: F, t8509: F, t8516: F) -> (F, F, F, F, F) {
    let t20090 = F::new(0.0005696928233656539) * t8464;
    let t20091 = F::new(3.5089340384731225) * t8482;
    let t20092 = F::new(51.94726769812759) * t8486;
    let t20094 = F::new(180.0) * t14435;
    let t20096 = -t20090 + t8469 + t8473 - t8477 + t11250 - t8481 + t20091 - t20092 + t8491 + t14432 + t14433 - F::new(0.4740006021527056) * t11254 - t8505 + t8509 + t20094 + F::new(3.1636214830824234) * t11256 + t14437 + t8516 + t14439;
    (t20090, t20091, t20092, t20094, t20096)
}
