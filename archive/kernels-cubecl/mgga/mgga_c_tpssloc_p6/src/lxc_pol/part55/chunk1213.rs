//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1213/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1213<F: Float>(t5107: F, t8320: F, t1774: F, t30991: F, t1799: F, t6995: F, t22574: F, t8643: F, t31048: F, t7685: F, t31033: F, t1983: F, t33136: F, t6996: F) -> (F, F, F, F, F, F) {
    let t119850 = F::cast_from(2.0_f64) * t8320 * t5107;
    let t119852 = F::cast_from(2.0_f64) * t30991 * t1774;
    let t119853 = t1799 * t6995;
    let t119856 = F::cast_from(6.0_f64) * t22574 * t8643 * t119853;
    let t119858 = F::cast_from(3.0_f64) * t7685 * t31048;
    let t119862 = t7685 * t31033;
    let t119867 = F::cast_from(2.0_f64) * t1983 * t6996 * t33136;
    (t119850, t119852, t119856, t119858, t119862, t119867)
}
