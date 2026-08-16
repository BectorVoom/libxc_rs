//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 851/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk851(t3034: f64, t725: f64, t41: f64, t5812: f64, t5815: f64, t5818: f64, t5821: f64, t5925: f64, t5936: f64, t5940: f64, t5945: f64, t5950: f64, t5959: f64, t5963: f64) -> f64 {
    let t9014 = t3034 * t725;
    let t9015 = t41 * t9014;
    let t9017 = t5812 + t5815 + t5925 - t9015 - t5818 + t5821 + 0.72290542002011598948e-2_f64 * t5936 + t5940 + t5945 - t5950 + t5959 + t5963;
    t9017
}
