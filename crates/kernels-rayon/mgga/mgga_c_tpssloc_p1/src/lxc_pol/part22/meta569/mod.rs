//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta569 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2076;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2077;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta569(t1032: f64, t10375: f64, t370: f64, t374: f64, t376: f64, t9697: f64, t10473: f64, t361: f64, t363: f64, t42342: f64, t42345: f64, t3131: f64, t221: f64, t339: f64, t42813: f64, t10216: f64, t2978: f64, t10479: f64, t42333: f64, t3061: f64, t676: f64, t11065: f64, t42387: f64, t1005: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t43248, t43253, t43288, t43291, t43292) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2076(t1032, t10375, t370, t374, t376, t9697, t10473, t361, t363, t42342, t42345, t3131);
        let (t43307, t43317, t43322, t43338, t43361, t43382) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2077(t221, t339, t42813, t10216, t2978, t10479, t42333, t3061, t676, t11065, t42387, t1005, t10375);
    (t43248, t43253, t43288, t43291, t43292, t43307, t43317, t43322, t43338, t43361, t43382)
}
