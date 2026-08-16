//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 273/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk273<F: Float>(t203: F, t447: F, t475: F, t1064: F, t366: F, t51: F, t372: F, t374: F, t365: F, t23: F, t6: F, t103: F, t61: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1065 = t203 * t447;
    let t1066 = t1065 * t475;
    let t1067 = t1064 * t1066;
    let t1070 = t366 * t51;
    let t1071 = F::cast_from(1.0_f64) / t1070;
    let t1072 = t372 * t372;
    let t1074 = t1071 * t1072 * t374;
    let t1076 = F::cast_from(0.11696446794910408142e1_f64) * t365 * t1074;
    let t1077 = t6 * t23;
    let t1079 = t61 * t1077 * t103;
    (t1065, t1066, t1067, t1071, t1072, t1074, t1076, t1077, t1079)
}
