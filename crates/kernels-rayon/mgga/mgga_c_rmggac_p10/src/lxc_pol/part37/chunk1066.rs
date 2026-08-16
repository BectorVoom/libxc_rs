//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1066/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1066(t74919: f64, t71196: f64, t71207: f64, t73309: f64, t74927: f64, t74929: f64, t74930: f64, t74932: f64, t77256: f64, t77258: f64, t77260: f64, t77265: f64, t77271: f64, t77275: f64, t77279: f64, t77280: f64, t77281: f64) -> f64 {
    let t80179 = 0.24527028530061914062e-5_f64 * t74919;
    let t80182 = -t77256 + t71196 + t80179 + t77258 + t73309 - t77260 + t77265 - t74927 + t74929 + 0.93188427318671584242e-2_f64 * t74930 - 0.15531404553111930707e-1_f64 * t74932 - t71207 - t77271 + t77275 + t77279 + t77280 - t77281;
    t80182
}
