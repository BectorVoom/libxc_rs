//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1034/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1034(t225: f64, t387: f64, t4657: f64, t345: f64, t7569: f64, t1921: f64, t25749: f64, t986: f64, t7593: f64, t990: f64, t25705: f64, t349: f64) -> (f64, f64, f64, f64, f64) {
    let t25766 = t4657 * t225 * t387;
    let t25767 = t345 * t25766;
    let t25778 = t7569 * t225;
    let t25784 = t1921 * t25749;
    let t25785 = t986 * t25784;
    let t25789 = t990 * t7593;
    let t25791 = t349 * t25705;
    (t25767, t25778, t25785, t25789, t25791)
}
