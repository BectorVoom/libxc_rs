//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2269/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2269(t25989: f64, t83886: f64, t25994: f64, t4034: f64, t15857: f64, t1873: f64, t652: f64, t1874: f64, t45632: f64, t26135: f64, t3941: f64, t671: f64) -> (f64, f64, f64, f64, f64) {
    let t91771 = 6.0_f64 * t83886 * t25989;
    let t91777 = 4.0_f64 * t4034 * t25994;
    let t91780 = 2.0_f64 * t652 * t15857 * t1873;
    let t91782 = 2.0_f64 * t45632 * t1874;
    let t91799 = 54.0_f64 * t3941 * t26135 * t671;
    (t91771, t91777, t91780, t91782, t91799)
}
