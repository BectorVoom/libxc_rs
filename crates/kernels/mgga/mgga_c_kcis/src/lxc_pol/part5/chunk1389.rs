//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1389/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1389<F: Float>(t1347: F, t6996: F, t1356: F, t21447: F, t1563: F, t7438: F, t11409: F, t12791: F, t16046: F, t16048: F, t16052: F, t17905: F, t21186: F, t21188: F, t21193: F, t21196: F, t21206: F, t21209: F, t21212: F, t21234: F, t21237: F, t21240: F, t21243: F) -> (F, F, F, F) {
    let t22888 = t6996 * t1347;
    let t22899 = t21447 * t1356;
    let t22904 = t7438 * t1563;
    let t22924 = -t12791 - F::cast_from(0.76103703703703703703e-2_f64) * t11409 - F::cast_from(0.1522074074074074074e-1_f64) * t16046 + F::cast_from(0.761037037037037037e-2_f64) * t16048 - t17905 - F::cast_from(0.2283111111111111111e-1_f64) * t16052 + F::cast_from(0.3805185185185185185e-2_f64) * t21186 - F::cast_from(0.19025925925925925925e-1_f64) * t21237 + F::cast_from(0.68493333333333333331e-1_f64) * t21234 + F::cast_from(0.4566222222222222222e-1_f64) * t21240 - F::cast_from(0.11415555555555555555e-1_f64) * t21188 - F::cast_from(0.10274e0_f64) * t21243 - F::cast_from(0.13698666666666666666e0_f64) * t21206 + F::cast_from(0.57077777777777777777e-2_f64) * t21196 - F::cast_from(0.11415555555555555555e-1_f64) * t21209 + F::cast_from(0.34246666666666666666e-1_f64) * t21212 - F::cast_from(0.17123333333333333333e-1_f64) * t21193;
    (t22888, t22899, t22904, t22924)
}
