//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1250/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1250<F: Float>(t10921: F, t10942: F, t10988: F, t11170: F, t39167: F, t39168: F, t39169: F, t39170: F, t39171: F, t39172: F, t39173: F, t39174: F, t39175: F, t39176: F, t39177: F, t40724: F, t40726: F, t40728: F, t40729: F, t40735: F, t41090: F, t41092: F, t41098: F, t8: F) -> F {
    let t41103 = -t39167 - t39168 - t10921 + t10942 + t39169 + t39170 + t39171 - t39172 + t11170 + t39173 - t39174 + t10988 - t39175 + t39176 + t39177 + t8 * (t40724 + t40726 + t40728 + t40729 + t40735 + t41090 + t41092 + t41098);
    t41103
}
