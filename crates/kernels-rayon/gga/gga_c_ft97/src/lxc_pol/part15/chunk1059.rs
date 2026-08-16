//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1059/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1059(t1013: f64, t12401: f64, t132: f64, t133: f64, t139: f64, t16798: f64, t16891: f64, t16907: f64, t2001: f64, t2058: f64, t20583: f64, t20631: f64, t20634: f64, t20651: f64, t3355: f64, t3392: f64, t3393: f64, t39802: f64, t4674: f64, t4702: f64, t4703: f64, t4710: f64, t4711: f64, t48636: f64, t527: f64, t550: f64, t62087: f64, t62090: f64, t76876: f64, t86694: f64, t86701: f64, t86708: f64, t86771: f64, t86824: f64, t86829: f64, t86850: f64, t86863: f64) -> f64 {
    let t86867 = -12.0_f64 * t2001 * t16907 * t4710 - 8.0_f64 * t2001 * t3355 * t20651 + 24.0_f64 * t133 * t39802 * t86694 - 36.0_f64 * t3392 * t16798 * t4710 + 6.0_f64 * t133 * t2058 * t86701 + 8.0_f64 * t3392 * t3393 * t20651 + 48.0_f64 * t2001 * t12401 * t86708 - 24.0_f64 * t62087 * t20583 - 48.0_f64 * t2001 * t48636 * t20634 + 24.0_f64 * t4674 * t4703 + 24.0_f64 * t2001 * t62090 * t4702 + 2.0_f64 * t527 * t139 * (t86771 + t86824) + 6.0_f64 * t86829 * t132 * t139 + 8.0_f64 * t16891 * t20631 - 12.0_f64 * t4674 * t4711 - 8.0_f64 * t2001 * t76876 * t1013 - t133 * t550 * (t86850 + t86863);
    t86867
}
