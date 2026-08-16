//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 799/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk799(t28636: f64, t28677: f64, t1055: f64, t1599: f64, t7561: f64, t25406: f64, t7565: f64, t1922: f64, t5838: f64, t1955: f64, t5919: f64, t10165: f64) -> (f64, f64, f64, f64, f64) {
    let t28678 = t28636 + t28677;
    let t28679 = t1055 * t28678;
    let t28681 = t1599 * t7561;
    let t28684 = t25406 * t7565;
    let t28691 = t5838 * t1922;
    let t28696 = t1955 * t5919;
    let t28697 = t10165 * t28696;
    (t28679, t28681, t28684, t28691, t28697)
}
