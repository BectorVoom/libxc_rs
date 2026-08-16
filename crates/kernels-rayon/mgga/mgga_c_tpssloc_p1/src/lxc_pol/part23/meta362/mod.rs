//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta362 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1161;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1162;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta362(t204: f64, t376: f64, t370: f64, t374: f64, t9697: f64, t10473: f64, t361: f64, t363: f64, t42342: f64, t42345: f64, t3131: f64, t221: f64, t339: f64, t42813: f64, t10216: f64, t2978: f64, t3061: f64, t676: f64, t11065: f64, t42387: f64, t10475: f64, t2770: f64, t283: f64, t61: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t43216, t43253, t43288, t43291, t43292, t43307) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1161(t204, t376, t370, t374, t9697, t10473, t361, t363, t42342, t42345, t3131, t221, t339, t42813);
        let (t43317, t43338, t43361, t43385, t43399) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1162(t10216, t2978, t3061, t676, t11065, t42387, t10475, t42342, t42345, t2770, t283, t61);
    (t43216, t43253, t43288, t43291, t43292, t43307, t43317, t43338, t43361, t43385, t43399)
}
