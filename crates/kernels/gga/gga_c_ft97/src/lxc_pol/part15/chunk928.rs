//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 928/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk928<F: Float>(t1013: F, t12401: F, t132: F, t133: F, t139: F, t16798: F, t16891: F, t16907: F, t2001: F, t2058: F, t20583: F, t20631: F, t20634: F, t20651: F, t3355: F, t3392: F, t3393: F, t39802: F, t4674: F, t4702: F, t4703: F, t4710: F, t4711: F, t48636: F, t527: F, t550: F, t62087: F, t62090: F, t76876: F, t86694: F, t86701: F, t86708: F, t86771: F, t86824: F, t86829: F, t86850: F, t86863: F) -> (F,) {
    let t86867 = -12.0 * t2001 * t16907 * t4710 - 8.0 * t2001 * t3355 * t20651 + 24.0 * t133 * t39802 * t86694 - 36.0 * t3392 * t16798 * t4710 + 6.0 * t133 * t2058 * t86701 + 8.0 * t3392 * t3393 * t20651 + 48.0 * t2001 * t12401 * t86708 - 24.0 * t62087 * t20583 - 48.0 * t2001 * t48636 * t20634 + 24.0 * t4674 * t4703 + 24.0 * t2001 * t62090 * t4702 + 2.0 * t527 * t139 * (t86771 + t86824) + 6.0 * t86829 * t132 * t139 + 8.0 * t16891 * t20631 - 12.0 * t4674 * t4711 - 8.0 * t2001 * t76876 * t1013 - t133 * t550 * (t86850 + t86863);
    (t86867,)
}
