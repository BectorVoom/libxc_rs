//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 874/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk874<F: Float>(t1008: F, t378: F, t32140: F, t1013: F, t23701: F, t23745: F, t23825: F, t23839: F, t32764: F, t32767: F, t32774: F, t32786: F, t32815: F, t32836: F, t34461: F, t34477: F, t34868: F, t34873: F, t34878: F, t34884: F, t34888: F, t8852: F) -> (F, F, F, F) {
    let t34906 = t378 * t1008;
    let t34907 = t32140 * t34906;
    let t34910 = t378 * t1013;
    let t34916 = -F::new(0.20527106943485609994e0) * t8852 * t34868 + F::new(0.18125821328051150223e0) * t23839 * t34873 - F::new(0.18125821328051150223e0) * t23825 * t34878 + t32786 + F::new(0.30209702213418583705e-1) * t23701 * t34461 - F::new(0.45306850413028723348e0) * t32815 * t34884 + F::new(0.22653425206514361674e0) * t23745 * t34888 + F::new(0.80027204934668021496e-1) * t32767 * t34907 - F::new(0.12004080740200203224e0) * t32774 * t32140 * t34910 + t32836 + F::new(0.26675734978222673832e-1) * t32764 * t34477;
    (t34906, t34907, t34910, t34916)
}
