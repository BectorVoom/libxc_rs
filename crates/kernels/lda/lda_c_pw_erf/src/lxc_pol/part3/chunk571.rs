//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 571/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk571<F: Float>(t3063: F, t62: F, t370: F, t973: F, t70: F, t990: F, t371: F, t67: F, t987: F, t991: F, t1005: F, t1013: F, t1023: F, t1031: F, t155: F, t174: F, t2748: F, t2752: F, t2755: F, t2759: F, t3020: F, t3027: F, t3031: F, t3038: F, t3046: F, t3059: F, t365: F, t372: F, t387: F, t966: F, t971: F, t974: F, t984: F, t989: F, t992: F) -> (F, F, F, F, F, F, F, F) {
    let t3064 = t62 * t3063;
    let t3065 = t973 * t370;
    let t3067 = F::new(1.0) / t990 / t70;
    let t3068 = t3065 * t3067;
    let t3071 = t3065 * t371;
    let t3075 = F::new(1.0) / t987 / t67;
    let t3076 = t62 * t3075;
    let t3077 = t3065 * t991;
    let t3080 = F::new(0.021687161765563047) * t174 * t3020 * t387 - F::new(0.016265371324172287) * t174 * t1005 * t1023 - F::new(0.4815944609513912) * t174 * t3027 * t1031 + t2748 - t2752 + t2755 + F::new(0.06849333333333334) * t174 * t3031 * t372 - F::new(0.05137) * t174 * t966 * t984 - F::new(1.6522997748472177) * t174 * t3038 * t992 + F::new(0.10274) * t174 * t155 * t971 * t974 - t2759 + F::new(0.032530742648344574) * t174 * t3046 * t1013 + F::new(1.0) * t365 * t3059 + F::new(2069.1336878655966) * t3064 * t3068 + F::new(6.0) * t989 * t3071 - F::new(192.9880990672242) * t3076 * t3077;
    (t3064, t3067, t3068, t3071, t3075, t3076, t3077, t3080)
}
