//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1796/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1796(t12188: f64, t12190: f64, t12194: f64, t12196: f64, t12200: f64, t1315: f64, t16101: f64, t19768: f64, t19771: f64, t19776: f64, t19779: f64, t19783: f64, t19787: f64, t5195: f64) -> f64 {
    let t19790 = -t12188 - 0.12962962962962962963e-1_f64 * t12190 - 0.24999999999999999999e-2_f64 * t19768 - 0.16666666666666666666e-2_f64 * t1315 * t19771 + 0.8333333333333333333e-3_f64 * t19776 - t12194 + t12196 - 0.52777777777777777776e-2_f64 * t12200 - 0.11666666666666666666e-1_f64 * t19779 - 0.19999999999999999999e-1_f64 * t16101 * t19783 + 0.99999999999999999996e-2_f64 * t5195 * t19787;
    t19790
}
