//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta214 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk909;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk910;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk911;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta214(t10619: f64, t324: f64, t300: f64, t2897: f64, t961: f64, t2940: f64, t2948: f64, t2928: f64, t941: f64, t2931: f64, t323: f64, t10524: f64, t959: f64, t10544: f64, t10530: f64, t10538: f64, t10556: f64, t10558: f64, t10560: f64, t10562: f64, t10566: f64, t10569: f64, t10572: f64, t10575: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10620, t10622, t10623, t10625, t10627, t10629) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk909(t10619, t324, t300, t2897, t961, t2940, t2948, t2928, t941);
        let t10632 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk910(t2931, t323);
        let (t10633, t10635, t10647) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk911(t10524, t10629, t10632, t959, t10544, t10530, t10538, t10556, t10558, t10560, t10562, t10566, t10569, t10572, t10575);
    (t10620, t10622, t10623, t10625, t10627, t10629, t10632, t10633, t10635, t10647)
}
