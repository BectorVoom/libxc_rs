//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 874/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk874(t1008: f64, t378: f64, t32140: f64, t1013: f64, t23701: f64, t23745: f64, t23825: f64, t23839: f64, t32764: f64, t32767: f64, t32774: f64, t32786: f64, t32815: f64, t32836: f64, t34461: f64, t34477: f64, t34868: f64, t34873: f64, t34878: f64, t34884: f64, t34888: f64, t8852: f64) -> (f64, f64, f64, f64) {
    let t34906 = t378 * t1008;
    let t34907 = t32140 * t34906;
    let t34910 = t378 * t1013;
    let t34916 = -0.20527106943485609994e0_f64 * t8852 * t34868 + 0.18125821328051150223e0_f64 * t23839 * t34873 - 0.18125821328051150223e0_f64 * t23825 * t34878 + t32786 + 0.30209702213418583705e-1_f64 * t23701 * t34461 - 0.45306850413028723348e0_f64 * t32815 * t34884 + 0.22653425206514361674e0_f64 * t23745 * t34888 + 0.80027204934668021496e-1_f64 * t32767 * t34907 - 0.12004080740200203224e0_f64 * t32774 * t32140 * t34910 + t32836 + 0.26675734978222673832e-1_f64 * t32764 * t34477;
    (t34906, t34907, t34910, t34916)
}
