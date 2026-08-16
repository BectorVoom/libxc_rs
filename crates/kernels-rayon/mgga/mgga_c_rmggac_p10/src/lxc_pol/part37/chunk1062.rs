//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1062/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1062(t68893: f64, t74730: f64, t74734: f64, t74739: f64, t74753: f64, t77172: f64, t77173: f64, t77174: f64, t77183: f64, t77184: f64, t77185: f64, t77186: f64, t77187: f64, t77189: f64, t77190: f64, t77191: f64, t77192: f64) -> f64 {
    let t80155 = t77172 + t77173 - t77174 - 0.17451485956252114153e-4_f64 * t74730 + 0.34902971912504228306e-4_f64 * t74734 + t68893 - 0.69805943825008456612e-4_f64 * t74739 + t77183 - t77184 - t77185 + t77186 + t77187 + 0.17451485956252114153e-4_f64 * t74753 + t77189 - t77190 + t77191 - t77192;
    t80155
}
