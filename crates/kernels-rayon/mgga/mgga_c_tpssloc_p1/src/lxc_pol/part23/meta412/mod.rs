//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta412 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1229;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1230;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta412(t16689: f64, t4101: f64, t16701: f64, t4205: f64, t20741: f64, t706: f64, t20234: f64, t751: f64, t9897: f64, t20742: f64, t67: f64, t758: f64, t12923: f64, t4194: f64, t5398: f64, t20800: f64, t262: f64, t10143: f64, t20778: f64, t13115: f64, t16586: f64, t21038: f64, t225: f64, t21061: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t67177, t67179, t67181, t67185, t67209) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1229(t16689, t4101, t16701, t4205, t20741, t706, t20234, t751, t9897, t20742, t67, t758);
        let (t67230, t67235, t67239, t67243, t67305, t67339) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1230(t12923, t4194, t5398, t20800, t262, t10143, t20778, t13115, t16586, t21038, t225, t21061);
    (t67177, t67179, t67181, t67185, t67209, t67230, t67235, t67239, t67243, t67305, t67339)
}
