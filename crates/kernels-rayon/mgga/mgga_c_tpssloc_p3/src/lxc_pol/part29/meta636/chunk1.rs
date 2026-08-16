//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2089/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2089(t22986: f64, t23270: f64, t865: f64, t86849: f64, t4300: f64, t776: f64, t857: f64, t1888: f64, t2717: f64, t25044: f64, t2742: f64, t23168: f64, t25342: f64) -> (f64, f64, f64, f64, f64) {
    let t86852 = t22986 * t23270 * t86849 * t865;
    let t86857 = t22986 * t23270 * t857 * t4300 * t776;
    let t86862 = t1888 * t23270 * t2717 * t4300 * t865;
    let t86866 = t1888 * t23270 * t25044 * t2742;
    let t86868 = t23168 * t25342;
    (t86852, t86857, t86862, t86866, t86868)
}
