//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1681/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1681(t225: f64, t5213: f64, t1807: f64, t3879: f64, t5211: f64, t1332: f64, t5343: f64, t1372: f64, t1824: f64, t5250: f64, t5286: f64, t562: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16022 = t5213 * t225;
    let t16028 = t1807 * t3879;
    let t16030 = t5211 * t225;
    let t16033 = t1332 * t5343;
    let t16036 = t1372 * t1824;
    let t16037 = t16036 * t5250;
    let t16040 = t562 * t5286;
    (t16022, t16028, t16030, t16033, t16036, t16037, t16040)
}
