//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 922/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk922(t168: f64, t2831: f64, t703: f64, t1072: f64, t1472: f64, t4550: f64, t4552: f64, t4554: f64, t4557: f64, t4566: f64, t4568: f64, t4600: f64, t5574: f64, t5577: f64, t5582: f64, t5588: f64, t5595: f64) -> f64 {
    let t8064 = 0.39794582218349216586e-1_f64 * t168 * t703 * t2831;
    let t8066 = t168 * t1472 * t1072;
    let t8075 = -t5595 + t8064 - 0.53059442957798955448e-1_f64 * t8066 - t4550 + 0.83762820535504401876e-1_f64 * t4552 + 0.33505128214201760751e0_f64 * t4554 + t4557 - 0.83762820535504401876e-1_f64 * t4566 - 0.16752564107100880375e0_f64 * t4568 - t4600 - 0.1061188859155979109e0_f64 * t5574 + 0.19897291109174608293e-1_f64 * t5577 - 0.3350512821420176075e0_f64 * t5582 + t5588;
    t8075
}
