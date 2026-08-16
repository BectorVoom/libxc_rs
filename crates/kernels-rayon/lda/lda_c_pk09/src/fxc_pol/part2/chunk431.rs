//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 431/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk431(t2171: f64, t2175: f64, t2179: f64, t761: f64, t762: f64, t760: f64, t772: f64, t131: f64, t200: f64, t205: f64, t2155: f64, t2183: f64, t2193: f64, t2198: f64, t2202: f64, t2206: f64, t2210: f64, t2214: f64, t571: f64, t575: f64, t723: f64, t727: f64, t739: f64, t750: f64, t752: f64, t754: f64, t98: f64) -> (f64, f64, f64, f64) {
    let t2220 = t761 + t762 + 1.5625_f64 * t2171 + 1.5625_f64 * t2175 - 1.5625_f64 * t2179;
    let t2221 = t760 * t2220;
    let t2222 = t2221 * t772;
    let t2225 = -22.07984838129906_f64 * t2155 + t571 + t575 - 2.427516195194328_f64 * t2183 * t98 - 0.5923479790153209_f64 * t727 * t131 * t2193 + 2.3693919160612835_f64 * t205 * t2198 + 2.3693919160612835_f64 * t205 * t2202 - 2.3693919160612835_f64 * t205 * t2206 + 2.427516195194328_f64 * t200 * t2210 + 2.427516195194328_f64 * t200 * t2214 - 2.9824072957409817_f64 * t2222 * t98 + t723 + t739 + t750 - t752 - t754;
    (t2220, t2221, t2222, t2225)
}
