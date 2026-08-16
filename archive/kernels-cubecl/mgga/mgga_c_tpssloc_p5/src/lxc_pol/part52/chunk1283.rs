//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1283/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1283<F: Float>(t118690: F, t22986: F, t6646: F, t829: F, t112968: F, t25261: F, t2647: F, t112974: F, t32849: F, t814: F, t32826: F, t6562: F, t794: F) -> (F, F, F, F, F, F) {
    let t118694 = F::cast_from(0.3289868133696452873e-1_f64) * t22986 * t6646 * t118690 * t829;
    let t118695 = F::cast_from(0.76763589786250567036e-1_f64) * t112968;
    let t118699 = F::cast_from(0.3289868133696452873e-1_f64) * t22986 * t6646 * t25261 * t2647;
    let t118700 = F::cast_from(0.38381794893125283518e-1_f64) * t112974;
    let t118705 = t814 * t32849;
    let t118709 = t6562 * t794 * t32826;
    (t118694, t118695, t118699, t118700, t118705, t118709)
}
