//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta548 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1817;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1818;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta548(t23196: f64, t23204: f64, t6562: f64, t6556: f64, t81632: f64, t23012: f64, t6573: f64, t1883: f64, t82045: f64, t23164: f64, t6555: f64, t82133: f64, t23197: f64, t6547: f64, t23257: f64, t794: f64, t6568: f64, t23205: f64, t82038: f64, t23242: f64, t81979: f64, t1081: f64, t2752: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t82182, t82209, t82211, t82218, t82221) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1817(t23196, t23204, t6562, t6556, t81632, t23012, t6573, t1883, t82045, t23164, t6555, t82133);
        let (t82230, t82236, t82259, t82294, t82296, t83555) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1818(t23197, t6547, t23257, t6562, t794, t23012, t6568, t23205, t82038, t23242, t81979, t1081, t2752);
    (t82182, t82209, t82211, t82218, t82221, t82230, t82236, t82259, t82294, t82296, t83555)
}
