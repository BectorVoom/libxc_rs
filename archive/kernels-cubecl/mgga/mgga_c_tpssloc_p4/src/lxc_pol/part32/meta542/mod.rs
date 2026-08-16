//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta542 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1887;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1888;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1889;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta542<F: Float>(t1215: F, t24815: F, t27637: F, t1210: F, t1734: F, t1011: F, t475: F, t1218: F, t1232: F, t1737: F, t1748: F, t24685: F, t24712: F, t24716: F, t24736: F, t27604: F, t27609: F, t27611: F, t27614: F, t27617: F, t27622: F, t27626: F, t27629: F, t27636: F, t7331: F, t8040: F, t1409: F, t2132: F, t2136: F, t460: F, t4928: F, t7320: F, t210: F, t7998: F, t1193: F, t8020: F, t1198: F, t2134: F, t24723: F, t24729: F, t24733: F, t24741: F, t4950: F, t4954: F, t4980: F, t4984: F, t5046: F, t7310: F, t7316: F, t7321: F, t8028: F, t8031: F, t8035: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t27638, t27639, t27642, t27644, t27645, t27648) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1887::<F>(t1215, t24815, t27637, t1210, t1734, t1011, t475, t1218, t1232, t1737, t1748, t24685, t24712, t24716, t24736, t27604, t27609, t27611, t27614, t27617, t27622, t27626, t27629, t27636, t7331, t8040);
        let (t27651, t27654, t27655, t27674) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1888::<F>(t1409, t2132, t2136, t460, t4928, t7320, t210, t7998);
        let (t27677, t27679) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1889::<F>(t1193, t8020, t1198, t2134, t24723, t24729, t24733, t24741, t27651, t27655, t27674, t4950, t4954, t4980, t4984, t5046, t7310, t7316, t7321, t8028, t8031, t8035);
    (t27638, t27639, t27642, t27644, t27645, t27648, t27651, t27654, t27655, t27674, t27677, t27679)
}
