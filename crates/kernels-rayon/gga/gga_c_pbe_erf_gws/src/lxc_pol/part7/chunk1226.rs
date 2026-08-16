//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1226/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1226(t20702: f64, t20712: f64, t20725: f64, t20731: f64, t20746: f64, t20750: f64, t20753: f64, t20755: f64, t20758: f64, t20761: f64, t20769: f64, t20781: f64, t20785: f64, t20791: f64, t20793: f64, t20797: f64, t20799: f64, t20801: f64, t20806: f64, t20829: f64, t20832: f64, t20837: f64) -> (f64, f64) {
    let t21696 = t20702 - t20712 - t20725 + t20731 + t20746 + t20750 + t20753 + t20755 + t20758 + t20761 - t20769;
    let t21697 = t20781 - t20785 + t20791 + t20793 + t20797 + t20799 + t20801 - t20806 + t20829 + t20832 - t20837;
    (t21696, t21697)
}
