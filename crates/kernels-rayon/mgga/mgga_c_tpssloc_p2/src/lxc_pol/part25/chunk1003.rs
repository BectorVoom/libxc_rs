//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1003/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1003(t1358: f64, t6940: f64, t1887: f64, t22715: f64, t534: f64, t1995: f64, t9223: f64, t213: f64, t1999: f64, t1372: f64, t552: f64, t1307: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22860 = t6940 * t1358;
    let t22863 = t22715 * t534 * t1887;
    let t22865 = t9223 * t1995;
    let t22866 = t22865 * t213;
    let t22867 = t22866 * t1999;
    let t22881 = t552 * t1372;
    let t22882 = t22881 * t1307;
    (t22860, t22863, t22865, t22867, t22881, t22882)
}
