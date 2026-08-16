//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1329/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1329(t29809: f64, t85639: f64, t1751: f64, t8034: f64, t29822: f64, t29624: f64, t491: f64, t27381: f64, t8009: f64, t29585: f64, t6686: f64, t29614: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t103130 = t85639 * t29809;
    let t103143 = t8034 * t1751;
    let t103149 = t85639 * t29822;
    let t103175 = t29624 * t491;
    let t103188 = t8009 * t27381;
    let t103218 = t29585 * t6686;
    let t103226 = t29614 * t491;
    (t103130, t103143, t103149, t103175, t103188, t103218, t103226)
}
