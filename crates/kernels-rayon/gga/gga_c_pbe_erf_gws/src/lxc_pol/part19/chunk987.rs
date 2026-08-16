//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 987/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk987(t10272: f64, t153: f64, t156: f64, t4550: f64, t4554: f64, t4557: f64, t4568: f64, t4600: f64, t5574: f64, t5582: f64, t5585: f64, t5588: f64, t5592: f64, t5595: f64, t8064: f64, t8066: f64) -> f64 {
    let t11178 = 0.13287210228946179141e1_f64 * t5585 + t5592 - t5595 + t8064 - 0.1061188859155979109e0_f64 * t8066 + 0.42708890021612718669e0_f64 * t153 * t156 * t10272 - t4550 + 0.16752564107100880375e0_f64 * t4554 + t4557 - 0.83762820535504401876e-1_f64 * t4568 - t4600 - 0.53059442957798955448e-1_f64 * t5574 - 0.16752564107100880375e0_f64 * t5582 + t5588;
    t11178
}
