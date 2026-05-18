//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 860/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk860<F: Float>(t1137: F, t1403: F, t1427: F, t247: F, t33499: F, t33573: F, t33589: F, t33592: F, t33594: F, t35547: F, t35550: F, t35573: F, t35605: F, t35640: F, t35679: F, t35693: F, t35706: F, t35729: F, t35738: F, t35744: F, t35753: F, t35779: F, t6749: F, t7558: F) -> F {
    let t35785 = -t33499 * t6749 / F::new(18.0) + F::new(2.0) * t35547 - t1403 * t35550 / F::new(3.0) - t247 * t35744 + F::new(4.0) * t35729 - F::new(12.0) * t35605 + F::new(8.0) * t35738 + F::new(8.0) * t35573 + F::new(4.0) * t35640 + t1403 * t35753 / F::new(6.0) - t1137 * t7558 - t33573 - t33589 + t33592 - t33594 + t35779 * t1427 / F::new(6.0) - F::new(2.0) * t35693 - F::new(2.0) * t35679 - F::new(2.0) * t35706;
    t35785
}
