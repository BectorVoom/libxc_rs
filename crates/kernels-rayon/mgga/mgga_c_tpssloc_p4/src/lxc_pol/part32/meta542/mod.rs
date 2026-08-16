//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta542 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1887;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1888;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1889;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta542(t1215: f64, t24815: f64, t27637: f64, t1210: f64, t1734: f64, t1011: f64, t475: f64, t1218: f64, t1232: f64, t1737: f64, t1748: f64, t24685: f64, t24712: f64, t24716: f64, t24736: f64, t27604: f64, t27609: f64, t27611: f64, t27614: f64, t27617: f64, t27622: f64, t27626: f64, t27629: f64, t27636: f64, t7331: f64, t8040: f64, t1409: f64, t2132: f64, t2136: f64, t460: f64, t4928: f64, t7320: f64, t210: f64, t7998: f64, t1193: f64, t8020: f64, t1198: f64, t2134: f64, t24723: f64, t24729: f64, t24733: f64, t24741: f64, t4950: f64, t4954: f64, t4980: f64, t4984: f64, t5046: f64, t7310: f64, t7316: f64, t7321: f64, t8028: f64, t8031: f64, t8035: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27638, t27639, t27642, t27644, t27645, t27648) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1887(t1215, t24815, t27637, t1210, t1734, t1011, t475, t1218, t1232, t1737, t1748, t24685, t24712, t24716, t24736, t27604, t27609, t27611, t27614, t27617, t27622, t27626, t27629, t27636, t7331, t8040);
        let (t27651, t27654, t27655, t27674) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1888(t1409, t2132, t2136, t460, t4928, t7320, t210, t7998);
        let (t27677, t27679) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1889(t1193, t8020, t1198, t2134, t24723, t24729, t24733, t24741, t27651, t27655, t27674, t4950, t4954, t4980, t4984, t5046, t7310, t7316, t7321, t8028, t8031, t8035);
    (t27638, t27639, t27642, t27644, t27645, t27648, t27651, t27654, t27655, t27674, t27677, t27679)
}
