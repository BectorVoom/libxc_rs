//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1389/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1389(t1347: f64, t6996: f64, t1356: f64, t21447: f64, t1563: f64, t7438: f64, t11409: f64, t12791: f64, t16046: f64, t16048: f64, t16052: f64, t17905: f64, t21186: f64, t21188: f64, t21193: f64, t21196: f64, t21206: f64, t21209: f64, t21212: f64, t21234: f64, t21237: f64, t21240: f64, t21243: f64) -> (f64, f64, f64, f64) {
    let t22888 = t6996 * t1347;
    let t22899 = t21447 * t1356;
    let t22904 = t7438 * t1563;
    let t22924 = -t12791 - 0.76103703703703703703e-2_f64 * t11409 - 0.1522074074074074074e-1_f64 * t16046 + 0.761037037037037037e-2_f64 * t16048 - t17905 - 0.2283111111111111111e-1_f64 * t16052 + 0.3805185185185185185e-2_f64 * t21186 - 0.19025925925925925925e-1_f64 * t21237 + 0.68493333333333333331e-1_f64 * t21234 + 0.4566222222222222222e-1_f64 * t21240 - 0.11415555555555555555e-1_f64 * t21188 - 0.10274e0_f64 * t21243 - 0.13698666666666666666e0_f64 * t21206 + 0.57077777777777777777e-2_f64 * t21196 - 0.11415555555555555555e-1_f64 * t21209 + 0.34246666666666666666e-1_f64 * t21212 - 0.17123333333333333333e-1_f64 * t21193;
    (t22888, t22899, t22904, t22924)
}
