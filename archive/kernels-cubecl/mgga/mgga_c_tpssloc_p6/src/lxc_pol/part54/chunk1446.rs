//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1446/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1446<F: Float>(t122675: F, t26161: F, t26163: F, t119853: F, t22574: F, t24432: F, t1983: F, t31035: F, t7940: F, t31304: F, t7754: F, t33366: F, t6876: F) -> (F, F, F, F, F) {
    let t122678 = F::cast_from(2.0_f64) * t26161 * t122675 * t26163;
    let t122681 = F::cast_from(3.0_f64) * t22574 * t24432 * t119853;
    let t122692 = t1983 * t7940 * t31035;
    let t122696 = t31304 * t7754;
    let t122697 = t6876 * t33366;
    (t122678, t122681, t122692, t122696, t122697)
}
