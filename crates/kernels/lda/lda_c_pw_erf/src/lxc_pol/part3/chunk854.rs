//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 854/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk854<F: Float>(t2702: F, t2708: F, t2711: F, t2738: F, t2747: F, t2751: F, t2754: F, t2758: F, t2760: F, t2943: F, t2949: F, t2988: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8099 = F::new(1.9263778438055648) * t2702;
    let t8101 = F::new(0.1301229705933783) * t2708;
    let t8102 = F::new(0.08674864706225219) * t2711;
    let t8103 = F::new(2.339289358982082) * t2738;
    let t8106 = F::new(3.436685857643691) * t2747;
    let t8107 = F::new(0.2849333333333333) * t2751;
    let t8108 = F::new(0.2137) * t2754;
    let t8109 = F::new(0.4274) * t2758;
    let t8110 = F::new(48.0) * t2760;
    let t8113 = F::new(14.03573615389249) * t2943;
    let t8114 = F::new(415.5781415850207) * t2949;
    let t8118 = F::new(4101.558808403118) * t2988;
    (t8099, t8101, t8102, t8103, t8106, t8107, t8108, t8109, t8110, t8113, t8114, t8118)
}
