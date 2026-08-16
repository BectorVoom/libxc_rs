//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 950/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk950(t3936: f64, t6362: f64, t10029: f64, t10166: f64, t10172: f64, t10174: f64, t10178: f64, t10181: f64, t10185: f64, t10187: f64, t1306: f64, t955: f64, t9844: f64, t9846: f64, t9849: f64, t9852: f64, t9855: f64, t9858: f64, t9862: f64, t9866: f64, t9870: f64, t9978: f64, t9980: f64) -> (f64, f64) {
    let t10370 = t3936 * t6362;
    let t10374 = 2.0_f64 * t10370 * t1306 * t955 - t10029 - t10166 - t10172 - t10174 + t10178 - t10181 - t10185 + t10187 - t9844 + t9846 + t9849 - t9852 - t9855 - t9858 + t9862 + t9866 + t9870 + t9978 + t9980;
    (t10370, t10374)
}
