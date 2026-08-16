//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1122/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1122(t34277: f64, t466: f64, t1653: f64, t32457: f64, t7362: f64, t1716: f64, t8891: f64, t7376: f64, t8082: f64, t7375: f64, t2147: f64, t8054: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34278 = t466 * t34277;
    let t34284 = t32457 * t1653;
    let t34285 = t7362 * t34284;
    let t34288 = t1716 * t8891;
    let t34291 = t8082 * t7376;
    let t34292 = t7375 * t34291;
    let t34295 = t2147 * t8054;
    (t34278, t34284, t34285, t34288, t34291, t34292, t34295)
}
