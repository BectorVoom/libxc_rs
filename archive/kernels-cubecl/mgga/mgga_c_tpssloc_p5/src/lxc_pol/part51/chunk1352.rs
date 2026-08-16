//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1352/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1352<F: Float>(t24465: F, t26550: F, t55353: F, t8657: F, t1983: F, t24990: F, t31758: F, t24991: F, t8607: F, t4026: F, t8595: F, t1442: F, t31518: F) -> (F, F, F, F, F, F) {
    let t120869 = F::cast_from(27.0_f64) * t24465 * t26550;
    let t120871 = F::cast_from(27.0_f64) * t55353 * t8657;
    let t120874 = F::cast_from(3.0_f64) * t1983 * t31758 * t24990;
    let t120876 = F::cast_from(3.0_f64) * t8607 * t24991;
    let t120877 = t4026 * t8595;
    let t120878 = t1442 * t31518;
    (t120869, t120871, t120874, t120876, t120877, t120878)
}
