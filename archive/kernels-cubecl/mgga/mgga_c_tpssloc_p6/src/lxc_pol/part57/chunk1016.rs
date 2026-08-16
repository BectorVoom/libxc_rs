//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1016/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1016<F: Float>(t5493: F, t652: F, t8595: F, t33620: F, t4028: F, t22574: F, t33357: F, t33899: F, t1983: F, t33136: F, t7940: F, t28817: F, t8607: F) -> (F, F, F, F, F) {
    let t128452 = F::cast_from(2.0_f64) * t652 * t8595 * t5493;
    let t128454 = F::cast_from(4.0_f64) * t4028 * t33620;
    let t128457 = F::cast_from(6.0_f64) * t22574 * t33899 * t33357;
    let t128460 = F::cast_from(2.0_f64) * t1983 * t7940 * t33136;
    let t128464 = F::cast_from(6.0_f64) * t8607 * t28817;
    (t128452, t128454, t128457, t128460, t128464)
}
