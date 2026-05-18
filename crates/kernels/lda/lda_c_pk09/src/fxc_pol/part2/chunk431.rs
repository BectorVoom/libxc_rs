//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 431/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk431<F: Float>(t2171: F, t2175: F, t2179: F, t761: F, t762: F, t760: F, t772: F, t131: F, t200: F, t205: F, t2155: F, t2183: F, t2193: F, t2198: F, t2202: F, t2206: F, t2210: F, t2214: F, t571: F, t575: F, t723: F, t727: F, t739: F, t750: F, t752: F, t754: F, t98: F) -> (F, F, F, F) {
    let t2220 = t761 + t762 + F::new(1.5625) * t2171 + F::new(1.5625) * t2175 - F::new(1.5625) * t2179;
    let t2221 = t760 * t2220;
    let t2222 = t2221 * t772;
    let t2225 = -F::new(22.07984838129906) * t2155 + t571 + t575 - F::new(2.427516195194328) * t2183 * t98 - F::new(0.5923479790153209) * t727 * t131 * t2193 + F::new(2.3693919160612835) * t205 * t2198 + F::new(2.3693919160612835) * t205 * t2202 - F::new(2.3693919160612835) * t205 * t2206 + F::new(2.427516195194328) * t200 * t2210 + F::new(2.427516195194328) * t200 * t2214 - F::new(2.9824072957409817) * t2222 * t98 + t723 + t739 + t750 - t752 - t754;
    (t2220, t2221, t2222, t2225)
}
