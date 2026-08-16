//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1045/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1045(t1065: f64, t1409: f64, t23330: f64, t23329: f64, t1945: f64, t4552: f64, t1603: f64, t6768: f64, t23384: f64, t7557: f64, t4693: f64, t6705: f64) -> (f64, f64, f64, f64, f64) {
    let t25814 = t1409 * t1065;
    let t25815 = t23330 * t25814;
    let t25816 = t23329 * t25815;
    let t25820 = t4552 * t1945;
    let t25822 = t1603 * t6768;
    let t25824 = t23384 * t7557;
    let t25826 = t6705 * t4693;
    (t25816, t25820, t25822, t25824, t25826)
}
