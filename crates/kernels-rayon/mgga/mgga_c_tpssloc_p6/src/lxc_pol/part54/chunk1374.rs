//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1374/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1374(t6562: f64, t8547: f64, t86893: f64, t214: f64, t7823: f64, t6552: f64, t6555: f64, t225: f64, t33412: f64, t1880: f64, t25224: f64, t31419: f64) -> (f64, f64, f64, f64, f64) {
    let t121399 = t6562 * t86893 * t8547;
    let t121401 = t214 * t7823;
    let t121403 = t6552 * t121401 * t6555;
    let t121405 = t33412 * t225;
    let t121409 = t1880 * t25224 * t31419;
    (t121399, t121401, t121403, t121405, t121409)
}
