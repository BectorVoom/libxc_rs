//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 740/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk740(t4688: f64, t4711: f64, t4714: f64, t4718: f64, t4799: f64, t4803: f64, t4807: f64, t4811: f64, t4815: f64, t4818: f64, t4820: f64, t4822: f64, t4824: f64, t4826: f64) -> f64 {
    let t6079 = -t4799 - t4803 + t4807 + t4811 - t4815 + t4688 + t4711 - t4714 - t4718 - t4818 + t4820 - t4822 + t4824 + t4826;
    t6079
}
