//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta353 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1148;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1149;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta353(t23076: f64, t241: f64, t67: f64, t2559: f64, t2570: f64, t782: f64, t9558: f64, t786: f64, t9569: f64, t222: f64, t39934: f64, t2691: f64, t812: f64, t815: f64, t238: f64, t244: f64, t248: f64, t40445: f64, t116: f64, t207: f64, t40419: f64, t9538: f64, t154: f64, t1891: f64, t205: f64, t792: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40971, t41008, t41011, t41083, t41096, t41115) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1148(t23076, t241, t67, t2559, t2570, t782, t9558, t786, t9569, t222, t39934, t2691, t812, t815);
        let (t41139, t41146, t41155, t41161, t41170) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1149(t238, t244, t248, t40445, t116, t207, t40419, t9538, t154, t1891, t205, t792, t9558);
    (t40971, t41008, t41011, t41083, t41096, t41115, t41139, t41146, t41155, t41161, t41170)
}
