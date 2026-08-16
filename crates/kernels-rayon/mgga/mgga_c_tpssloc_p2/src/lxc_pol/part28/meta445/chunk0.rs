//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1629/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1629(t776: f64, t857: f64, t865: f64, t23270: f64, t22986: f64, t25: f64, t2749: f64, t606: f64, t868: f64, t2745: f64, t2379: f64, t28: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t23272 = t857 * t776 * t865;
    let t23273 = t23270 * t23272;
    let t23274 = t22986 * t23273;
    let t23296 = t25 * t2749;
    let t23299 = t606 * t868;
    let t23302 = t25 * t2745;
    let t23781 = t28 * t2379;
    (t23272, t23273, t23274, t23296, t23299, t23302, t23781)
}
