//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1128/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1128(t27480: f64, t27529: f64, t27568: f64, t27739: f64, t1241: f64, t2154: f64, t5088: f64, t3598: f64, t1751: f64, t7299: f64, t7302: f64, t24574: f64, t8015: f64) -> (f64, f64, f64, f64) {
    let t27741 = t27480 + t27529 + t27568 + t27739;
    let t27742 = t1241 * t27741;
    let t27746 = t2154 * t5088;
    let t27747 = t3598 * t27746;
    let t27751 = t7299 * t1751;
    let t27752 = t27751 * t7302;
    let t27755 = t24574 * t8015;
    (t27742, t27747, t27752, t27755)
}
