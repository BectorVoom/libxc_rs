//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta616 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2093;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2094;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta616(t10984: f64, t6717: f64, t1036: f64, t23557: f64, t1933: f64, t1937: f64, t2250: f64, t3200: f64, t83015: f64, t1030: f64, t1058: f64, t3068: f64, sigma0: f64, t25511: f64, t6743: f64, t23592: f64, t23631: f64, t974: f64, t25721: f64, t210: f64, t23599: f64, t23632: f64, t23511: f64, t23634: f64, t23518: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t83167, t83172, t83206, t83215, t83220) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2093(t10984, t6717, t1036, t23557, t1933, t1937, t2250, t3200, t83015, t1030, t1058, t3068, sigma0);
        let (t83233, t83239, t83240, t83244, t83245, t83246, t83265) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2094(t25511, t6743, t23592, t23631, t974, t25721, t210, t23599, t23632, t23511, t23634, t23518);
    (t83167, t83172, t83206, t83215, t83220, t83233, t83239, t83240, t83244, t83245, t83246, t83265)
}
