//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta324 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1399;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1400;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta324(t11651: f64, t3515: f64, t3576: f64, t3604: f64, t3585: f64, t820: f64, t10401: f64, t3575: f64, t3610: f64, t3624: f64, t3521: f64, t3579: f64, t3577: f64, t248: f64, t3494: f64, t3570: f64, t1213: f64, t3490: f64, t3523: f64, t1190: f64, t3030: f64, t3032: f64, t3505: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11652, t11665, t11668, t11678, t11692, t11697, t11698) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1399(t11651, t3515, t3576, t3604, t3585, t820, t10401, t3575, t3610, t3624, t3521, t3579);
        let (t11699, t11703, t11705, t11707, t11708, t11709) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1400(t11698, t3577, t248, t3494, t3570, t1213, t3490, t3523, t1190, t3030, t3032, t3505);
    (t11652, t11665, t11668, t11678, t11692, t11697, t11699, t11703, t11705, t11707, t11708, t11709)
}
