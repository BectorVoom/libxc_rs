//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 379/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk379(t120: f64, t1595: f64, t72: f64, t123: f64, t29: f64, t532: f64, t126: f64, t1631: f64, t1655: f64, t2009: f64, t2012: f64, t2014: f64, t534: f64) -> (f64, f64, f64) {
    let t2015 = t1595 * t120;
    let t2016 = t72 * t2015;
    let t2021 = t123 / t532 / t29;
    let t2022 = t1595 * t126;
    let t2030 = -0.11705142615505742e0_f64 * t2009 + 0.23410285231011484e0_f64 * t2012 - 0.26564305359272358183e-2_f64 * t2014 * t2016 + 0.319782988780431561e-1_f64 * t2021 * t2022 - 0.532971647967385935e-1_f64 * t534 * t1655 * t126 + 0.13977476158628290272e-1_f64 * t1631 * t2022;
    (t2015, t2021, t2030)
}
