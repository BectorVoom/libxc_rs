//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2630/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2630<F: Float>(t11583: F, t21510: F, t11570: F, t15376: F, t15382: F, t18484: F, t3447: F, t44478: F, t4919: F, t5979: F, t64648: F, t64951: F, t64969: F, t64976: F, t64979: F, t64981: F, t64988: F, t65077: F, t7319: F) -> (F, F, F) {
    let t73444 = t11583 * t21510;
    let t73451 = t11570 * t21510;
    let t73480 = F::cast_from(0.83333333333333333331e-3_f64) * t3447 * t4919 * t7319 * t5979 + F::cast_from(0.16666666666666666666e-2_f64) * t3447 * t4919 * t65077 - F::cast_from(0.81481481481481481478e-2_f64) * t64951 + F::cast_from(0.59259259259259259257e-2_f64) * t15376 * t18484 - F::cast_from(0.11111111111111111111e-2_f64) * t3447 * t64648 * t15382 - F::cast_from(0.3086419753086419753e-3_f64) * t44478 - F::cast_from(0.83333333333333333331e-3_f64) * t64969 - F::cast_from(0.27160493827160493826e-2_f64) * t64976 + F::cast_from(0.18518518518518518518e-3_f64) * t64979 + F::cast_from(0.44444444444444444443e-2_f64) * t64981 - F::cast_from(0.27777777777777777777e-3_f64) * t64988;
    (t73444, t73451, t73480)
}
