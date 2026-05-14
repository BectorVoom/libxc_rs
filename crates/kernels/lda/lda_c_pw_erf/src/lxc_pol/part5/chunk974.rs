//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 974/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk974<F: Float>(t1553: F, t2642: F, t169: F, t301: F, t717: F, t7387: F, t684: F, t7339: F, t142: F, t14473: F, t14485: F, t14488: F, t1550: F, t1554: F, t1555: F, t18805: F, t1881: F, t18901: F, t18906: F, t19449: F, t2211: F, t2592: F, t2765: F, t2805: F, t411: F, t440: F, t5735: F, t5783: F, t6025: F, t6098: F, t7166: F, t7214: F, t777: F, t7880: F, t7977: F, t7986: F, t7988: F, t7991: F) -> (F,) {
    let t20589 = t1553 * t2642;
    let t20608 = t169 * t717 * t7387 * t301;
    let t20618 = t684 * t7339;
    let t20628 = -t14473 + 9.0 * t5735 * t6098 - t777 * t20589 * t1555 + 9.0 * t2211 * t18906 - t777 * t2805 * t7991 + 9.0 * t2211 * t18901 - t777 * t1554 * t142 * t7166 + 2.0 * t1881 * t7988 + 2.0 * t7214 * t2592 + t7977 * t1550 - 0.054045904796391424 * t20608 - 9.0 * t5783 * t18805 + 6.0 * t14485 * t2765 * t7986 * t411 + 18.0 * t6025 * t19449 + 0.019957056683757683 * t20618 - 18.0 * t14488 * t2765 * t7880 * t411 + 18.0 * t14485 * t2765 * t7880 * t440;
    (t20628,)
}
