//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2924/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2924(t10523: f64, t2933: f64, t5790: f64, t959: f64, t14662: f64, t193: f64, t3216: f64, t336: f64, t4700: f64, t4701: f64, t59891: f64, t59958: f64, t59961: f64, t59966: f64, t59968: f64, t59970: f64, t59972: f64, t60880: f64, t60886: f64, t60890: f64, t60893: f64, t60899: f64) -> (f64, f64) {
    let t60903 = 0.10389515463408878255e3_f64 * t959 * t10523 * t5790 * t2933;
    let t60904 = -2.0_f64 * t193 * t3216 * t336 * t60880 - 2.0_f64 * t14662 * t4700 * t4701 - t59891 + t59958 + t59961 + t59966 + t59968 + t59970 - t59972 + t60886 - t60890 + t60893 - t60899 + t60903;
    (t60903, t60904)
}
