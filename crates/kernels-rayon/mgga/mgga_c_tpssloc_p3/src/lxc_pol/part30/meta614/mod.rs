//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta614 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2012;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2013;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta614(t3186: f64, t83015: f64, t3158: f64, t6712: f64, t10383: f64, t1926: f64, t10948: f64, t23536: f64, t10472: f64, t10474: f64, t10478: f64, t23535: f64, sigma0: f64, t23540: f64, t6753: f64, t10375: f64, t1942: f64, t23488: f64, t23509: f64, t23508: f64, t6721: f64, t6741: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t83016, t83025, t83028, t83043, t83054, t83058) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2012(t3186, t83015, t3158, t6712, t10383, t1926, t10948, t23536, t10472, t10474, t10478, t23535, sigma0);
        let (t83061, t83065, t83080, t83117, t83121) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2013(t10948, t23540, t10472, t10478, t6753, t10375, t1942, t23488, t23509, t23508, t6721, t6741);
    (t83016, t83025, t83028, t83043, t83054, t83058, t83061, t83065, t83080, t83117, t83121)
}
