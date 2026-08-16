//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta612 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2086;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2087;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta612(t1012: f64, t10515: f64, t6753: f64, t1933: f64, t23479: f64, t82916: f64, t23413: f64, t344: f64, t6740: f64, t1016: f64, t3034: f64, t1930: f64, t1015: f64, t23472: f64, t23503: f64, t10423: f64, t23419: f64, t23418: f64, t3180: f64, t10401: f64, t23417: f64, t3186: f64, t3158: f64, t6712: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t82964, t82971, t82981, t82986) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2086(t1012, t10515, t6753, t1933, t23479, t82916, t23413, t344, t6740, t1016, t3034, t1930);
        let (t82996, t83004, t83008, t83015, t83016, t83025) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2087(t1015, t23472, t23503, t10423, t23419, t23418, t3180, t10401, t23417, t3186, t3158, t6712);
    (t82964, t82971, t82981, t82986, t82996, t83004, t83008, t83015, t83016, t83025)
}
