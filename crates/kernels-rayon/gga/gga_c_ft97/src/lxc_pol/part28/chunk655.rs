//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 655/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk655(t432: f64, t452: f64, t6564: f64, t1332: f64, t1570: f64, t3188: f64, t11472: f64, t1557: f64, t11556: f64, t379: f64, t6538: f64, t8557: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26432 = t452 * t6564 * t432;
    let t26435 = t1332 * t1570;
    let t26436 = t26435 * t3188;
    let t26437 = t11472 * t26436;
    let t26440 = t1332 * t1557;
    let t26441 = t26440 * t3188;
    let t26442 = t11556 * t26441;
    let t26445 = t6538 * t379;
    let t26446 = t8557 * t26445;
    (t26432, t26436, t26437, t26441, t26442, t26445, t26446)
}
