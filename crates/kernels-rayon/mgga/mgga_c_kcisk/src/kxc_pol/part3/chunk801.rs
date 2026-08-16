//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 801/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk801(t11764: f64, t11767: f64, t11770: f64, t11772: f64, t11778: f64, t11782: f64, t11784: f64, t11787: f64, t11790: f64, t11792: f64, t11796: f64, t11800: f64, t11804: f64, t11810: f64, t11813: f64, t11925: f64, t11930: f64, t11932: f64) -> f64 {
    let t12396 = -0.375e0_f64 * t11764 - 0.62499999999999999999e-1_f64 * t11767 - 0.60703125e-1_f64 * t11770 - 0.625e-1_f64 * t11772 - 0.5625e0_f64 * t11778 - 0.13489583333333333333e-1_f64 * t11782 + 0.303515625e-1_f64 * t11784 + 0.40468749999999999999e-1_f64 * t11787 + 0.13489583333333333333e-1_f64 * t11790 - 0.28125e0_f64 * t11792 + 0.27777777777777777777e-1_f64 * t11796 - 0.28125e0_f64 * t11800 - 0.9375e-1_f64 * t11804 + 0.29976851851851851851e-2_f64 * t11810 - 0.13489583333333333333e-1_f64 * t11813 + 0.9375e-1_f64 * t11925 + 0.60703125e-1_f64 * t11930 + 0.1875e0_f64 * t11932;
    t12396
}
