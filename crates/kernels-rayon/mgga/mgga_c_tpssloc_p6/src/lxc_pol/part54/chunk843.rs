//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 843/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk843(t466: f64, t8054: f64, t1760: f64, t2154: f64, t3598: f64, t1653: f64, t7363: f64, t7362: f64, t1716: f64, t2148: f64, t1755: f64, t7376: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8055 = t466 * t8054;
    let t8060 = t2154 * t1760;
    let t8061 = t3598 * t8060;
    let t8066 = t7363 * t1653;
    let t8067 = t7362 * t8066;
    let t8070 = t1716 * t2148;
    let t8073 = t1755 * t7376;
    (t8055, t8061, t8066, t8067, t8070, t8073)
}
