//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 828/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk828(t8360: f64, t8373: f64, t862: f64, t89: f64, t161: f64, t3847: f64, t3855: f64, t3906: f64, t3908: f64, t3983: f64, t3984: f64, t3986: f64, t3993: f64, t4001: f64, t4005: f64, t7706: f64, t7776: f64, t98: f64) -> (f64, f64) {
    let t8374 = t8360 + t8373;
    let t8375 = t8374 * t862;
    let t8376 = t8375 * t89;
    let t8385 = -t3847 - t3855 - 4.738783832122567_f64 * t3906 - 3.7610742193750633_f64 * t3908 + 4.937333717448355_f64 * t8376 * t98 - 4.937333717448355_f64 * t161 * t7776 - 4.937333717448355_f64 * t161 * t7706 + t3983 - 1.4760499452555382_f64 * t3984 - 1.4760499452555382_f64 * t3986 + t3993 - t4001 - t4005;
    (t8374, t8385)
}
