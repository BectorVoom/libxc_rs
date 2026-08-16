//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1183/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1183(t47855: f64, t47862: f64, t47864: f64, t47866: f64, t47868: f64, t47870: f64, t47872: f64, t47874: f64, t47878: f64, t47882: f64, t47886: f64, t47888: f64, t47890: f64, t47892: f64, t47893: f64, t47895: f64, t47896: f64, t47898: f64, t47899: f64, t47902: f64, t47904: f64, t47906: f64) -> (f64, f64) {
    let t48656 = -t47855 + t47862 - t47864 + t47866 - t47868 + t47870 + t47872 - t47874 + t47878 + t47882 + t47886;
    let t48657 = -t47888 - t47890 - t47892 - t47893 - t47895 + t47896 - t47898 - t47899 - t47902 - t47904 - t47906;
    (t48656, t48657)
}
