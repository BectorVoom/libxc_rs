//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 769/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk769<F: Float>(t1449: F, t6061: F, t729: F, t762: F, t713: F, t7560: F, t265: F, t33452: F, t1901: F, t193: F, t33693: F, t33697: F, t33701: F, t33707: F, t33709: F, t33712: F, t33717: F, t33721: F, t33725: F, t446: F, t89: F) -> (F, F, F, F, F) {
    let t33728 = t6061 * t1449;
    let t33730 = t729 * t762 * t33728;
    let t33734 = t729 * t7560 * t713;
    let t33738 = t729 * t265 * t33452;
    let t33741 = -F::new(4.0) / F::new(3.0) * t1901 * t33693 - F::new(4.0) / F::new(3.0) * t1901 * t33697 + t89 * t193 * t33701 / F::new(3.0) - t33707 - F::new(2.0) / F::new(9.0) * t1901 * t33709 + F::new(2.0) / F::new(9.0) * t1901 * t33712 + t1901 * t33717 / F::new(9.0) + F::new(2.0) / F::new(3.0) * t446 * t33721 + F::new(2.0) / F::new(3.0) * t446 * t33725 + F::new(2.0) / F::new(3.0) * t446 * t33730 - t446 * t33734 / F::new(3.0) - t446 * t33738 / F::new(3.0);
    (t33728, t33730, t33734, t33738, t33741)
}
