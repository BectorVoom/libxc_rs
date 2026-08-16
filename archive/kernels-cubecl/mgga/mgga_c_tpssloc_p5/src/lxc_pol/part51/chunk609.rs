//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 609/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk609<F: Float>(t423: F, t4737: F, t1098: F, t1657: F, t1119: F, t1671: F, t3259: F, t1117: F, t3264: F, t1661: F, t3270: F, t1102: F) -> (F, F, F, F, F) {
    let t4739 = F::cast_from(0.621814e-1_f64) * t4737 * t423;
    let t4740 = t1657 * t1098;
    let t4742 = F::cast_from(1.0_f64) * t4740 * t1119;
    let t4744 = F::cast_from(1.0_f64) * t3259 * t1671;
    let t4745 = t1671 * t1117;
    let t4747 = F::cast_from(2.0_f64) * t3264 * t4745;
    let t4748 = t3270 * t1661;
    let t4749 = t4748 * t1102;
    (t4739, t4742, t4744, t4747, t4749)
}
