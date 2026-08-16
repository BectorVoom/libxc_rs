//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1937/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1937(t1055: f64, t21662: f64, t1603: f64, t5914: f64, t1634: f64, t5919: f64, t10165: f64, t21480: f64, t381: f64, t1625: f64, t5848: f64, t21614: f64, t349: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21663 = t1055 * t21662;
    let t21669 = t1603 * t5914;
    let t21676 = t5919 * t1634;
    let t21677 = t10165 * t21676;
    let t21682 = t21480 * t381;
    let t21684 = t5848 * t1625;
    let t21689 = t349 * t21614;
    (t21663, t21669, t21676, t21677, t21682, t21684, t21689)
}
