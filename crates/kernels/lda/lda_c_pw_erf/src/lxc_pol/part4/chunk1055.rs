//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1055/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1055<F: Float>(t43: F, t11317: F, t11319: F, t11322: F, t1055: F, t5967: F, t402: F, t6011: F, t75: F, t1051: F, t390: F, t40: F, t34: F, t348: F, t462: F, t1063: F, t11430: F, t1781: F, t2325: F, t2329: F, t2849: F, t2953: F, t35: F, t4352: F, t5982: F, t5987: F, t5992: F, t8315: F, t939: F, t940: F, t945: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t15338 = 0.032530742648344574 * t11317;
    let t15339 = 0.9631889219027824 * t11319;
    let t15340 = 0.043374323531126094 * t11322;
    let t15341 = t5967 * t1055;
    let t15342 = 17.315755899375862 * t15341;
    let t15344 = t6011 * t75 * t402;
    let t15345 = 1.169644679491041 * t15344;
    let t15346 = t5967 * t1051;
    let t15347 = 0.5848223397455204 * t15346;
    let t15349 = t40 * t6011 * t390;
    let t15350 = 2.0 * t15349;
    let t15355 = t348 * t34 * t462;
    let t15376 = piecewise3(t44, 0.0, 40.0 / 81.0 * t8315 * t2325 * t940 - 64.0 / 27.0 * t4352 * t15355 - 8.0 / 27.0 * t5982 * t945 + 32.0 / 9.0 * t939 * t35 * t1063 + 16.0 / 9.0 * t1781 * t462 - 16.0 / 3.0 * t1781 * t2849 - 8.0 / 27.0 * t2953 * t2329 * t940 + 8.0 / 9.0 * t939 * t5992 * t348 + 4.0 / 9.0 * t5987 * t945 + t11430);
    (t15338, t15339, t15340, t15342, t15345, t15347, t15350, t15355, t15376)
}
