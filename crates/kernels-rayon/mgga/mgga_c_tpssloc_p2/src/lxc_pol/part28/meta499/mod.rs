//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta499 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1723;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1724;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1725;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1726;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta499(t225: f64, t7824: f64, t1527: f64, t7106: f64, t2718: f64, t7823: f64, t798: f64, t25211: f64, t7815: f64, t1528: f64, t24297: f64, t25206: f64, t25209: f64, t25214: f64, t25218: f64, t25226: f64, t25230: f64, t259: f64, t2597: f64, t7842: f64, t855: f64, t866: f64, t218: f64, t26653: f64, t25346: f64, t10109: f64, t2053: f64, t4272: f64, t2047: f64, t4142: f64, t1492: f64, t7084: f64, t13042: f64, t13053: f64, t13065: f64, t2054: f64, t23250: f64, t23254: f64, t24318: f64, t24321: f64, t25168: f64, t25339: f64, t25343: f64, t26684: f64, t26698: f64, t870: f64, t2752: f64, t7844: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26700, t26703, t26708, t26713, t26719) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1723(t225, t7824, t1527, t7106, t2718, t7823, t798, t25211, t7815, t1528, t24297, t25206, t25209, t25214, t25218, t25226, t25230, t259, t2597, t7842, t855, t866);
        let (t26722, t26728, t26729, t26732, t26734, t26737) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1724(t218, t26653, t25346, t10109, t2053, t4272, t2047, t4142, t1492, t7084, t13042, t13053, t13065, t2054, t23250, t23254, t24318, t24321, t25168, t25339, t25343, t259);
        let (t26739, t26740) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1725(t26684, t26698, t26719, t26737, t870);
        let t26744 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1726(t2752, t7844);
    (t26700, t26703, t26708, t26713, t26722, t26728, t26729, t26732, t26734, t26739, t26740, t26744)
}
