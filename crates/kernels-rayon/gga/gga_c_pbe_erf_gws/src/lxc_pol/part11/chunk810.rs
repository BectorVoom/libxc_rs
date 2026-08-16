//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 810/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk810(t12774: f64, t12775: f64, t12777: f64, t12781: f64, t12785: f64, t12786: f64, t12787: f64, t12788: f64, t12789: f64, t12790: f64, t12791: f64, t12792: f64, t12793: f64, t12796: f64, t12799: f64, t5436: f64, t5443: f64, t5521: f64) -> f64 {
    let t13025 = t5436 - t5443 + t12774 + t12775 + t12777 + t12781 - t12785 - t12786 + t12787 + t12788 - t12789 + t12790 + t12791 - t5521 - t12792 - t12793 - t12796 + t12799;
    t13025
}
