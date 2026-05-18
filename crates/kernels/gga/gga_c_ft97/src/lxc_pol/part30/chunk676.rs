//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 676/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk676<F: Float>(t25044: F, t992: F, t2665: F, t446: F, t28719: F, t852: F, t1486: F, t193: F, t681: F, t7083: F, t89: F, t24995: F, t25010: F, t28770: F, t28774: F, t28779: F, t28783: F, t28784: F, t28790: F, t28794: F) -> (F, F, F, F, F, F) {
    let t28796 = t25044 * t992;
    let t28797 = t2665 * t28796;
    let t28798 = t446 * t28797;
    let t28800 = t852 * t28719;
    let t28802 = t1486 * t193 * t28800;
    let t28804 = t681 * t7083;
    let t28805 = t89 * t28804;
    let t28807 = -t28770 / F::new(9.0) + t28774 / F::new(27.0) - t28779 / F::new(6.0) - t28783 - t28784 / F::new(54.0) + t24995 / F::new(9.0) - t25010 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t28790 - F::new(2.0) * t28794 + t28798 / F::new(9.0) - t28802 / F::new(6.0) - F::new(2.0) / F::new(9.0) * t28805;
    (t28796, t28798, t28802, t28804, t28805, t28807)
}
