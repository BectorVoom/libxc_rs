//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1282/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1282(t1464: f64, t1497: f64, t27387: f64, t59071: f64, t101938: f64, t101941: f64, t101944: f64, t101948: f64, t101950: f64, t101954: f64, t101957: f64, t101959: f64, t101961: f64, t101965: f64, t7971: f64, t99035: f64) -> (f64, f64) {
    let t101969 = t1464 * t27387 * t59071 * t1497;
    let t101971 = -0.46429444444444444444e-2_f64 * t101938 + 0.38691203703703703703e-2_f64 * t101941 + t99035 - 0.23168402777777777778e-3_f64 * t101944 - 0.23214722222222222222e-2_f64 * t101948 + 0.33980324074074074074e-2_f64 * t101950 * t7971 - 0.23214722222222222222e-2_f64 * t101954 - 0.77382407407407407407e-3_f64 * t101957 - 0.46377350260416666667e-4_f64 * t101959 + 0.30918233506944444445e-4_f64 * t101961 + 0.34822083333333333332e-2_f64 * t101965 - 0.11607361111111111111e-2_f64 * t101969;
    (t101969, t101971)
}
