//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 947/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk947(t112: f64, t33627: f64, t1851: f64, t8660: f64, t2098: f64, t7774: f64, t580: f64, t1858: f64, t8646: f64, t2105: f64, t7758: f64, t2029: f64, t7945: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t122811 = t33627 * t112;
    let t122852 = t1851 * t8660;
    let t122853 = t2098 * t7774;
    let t122856 = t33627 * t580;
    let t122857 = t8646 * t1858;
    let t122860 = t7758 * t2105;
    let t122862 = t7945 * t2029;
    (t122811, t122852, t122853, t122856, t122857, t122860, t122862)
}
