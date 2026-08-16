//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta117 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk603;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk604;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta117(t1409: f64, t3242: f64, t3247: f64, t1098: f64, t1657: f64, t1661: f64, t3270: f64, t3287: f64, t1667: f64, t699: f64, t1128: f64, t1675: f64, t1147: f64, t1687: f64, t1694: f64, t3403: f64, t300: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4723, t4728, t4740, t4748, t4764, t4770, t4797) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk603(t1409, t3242, t3247, t1098, t1657, t1661, t3270, t3287, t1667, t699, t1128, t1675);
        let (t4835, t4861, t4869) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk604(t1147, t1687, t1694, t3403, t300);
    (t4723, t4728, t4740, t4748, t4764, t4770, t4797, t4835, t4861, t4869)
}
