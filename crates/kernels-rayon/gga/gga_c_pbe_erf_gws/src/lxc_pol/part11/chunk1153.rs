//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1153/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1153(t25049: f64, t30955: f64, t30957: f64, t30962: f64, t40954: f64, t40956: f64, t40958: f64, t40960: f64, t40989: f64, t47931: f64, t48006: f64, t48034: f64) -> f64 {
    let t48354 = 0.18891666666666666667e-2_f64 * t47931 + 0.33585185185185185186e-2_f64 * t30955 - 0.25188888888888888889e-2_f64 * t40954 + 0.15113333333333333333e-1_f64 * t40989 - 0.78365432098765432099e-2_f64 * t25049 + 0.50377777777777777778e-2_f64 * t30957 - 0.2518888888888888889e-1_f64 * t48034 + 0.12594444444444444445e-1_f64 * t48006 - 0.27987654320987654323e-2_f64 * t40956 + 0.10075555555555555556e-1_f64 * t40958 - 0.15113333333333333333e-1_f64 * t40960 - 0.10075555555555555556e-1_f64 * t30962;
    t48354
}
