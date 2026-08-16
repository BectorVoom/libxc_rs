//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta486 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1656;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1657;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1658;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1659;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta486<F: Float>(t225: F, t7824: F, t1527: F, t7106: F, t2718: F, t7823: F, t798: F, t25211: F, t7815: F, t1528: F, t24297: F, t25206: F, t25209: F, t25214: F, t25218: F, t25226: F, t25230: F, t259: F, t2597: F, t7842: F, t855: F, t866: F, t218: F, t26653: F, t25346: F, t10109: F, t2053: F, t4272: F, t2047: F, t4142: F, t1492: F, t7084: F, t13042: F, t13053: F, t13065: F, t2054: F, t23250: F, t23254: F, t24318: F, t24321: F, t25168: F, t25339: F, t25343: F, t26684: F, t26698: F, t870: F, t2752: F, t7844: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t26700, t26703, t26708, t26713, t26719) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1656::<F>(t225, t7824, t1527, t7106, t2718, t7823, t798, t25211, t7815, t1528, t24297, t25206, t25209, t25214, t25218, t25226, t25230, t259, t2597, t7842, t855, t866);
        let (t26722, t26728, t26729, t26732, t26734, t26737) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1657::<F>(t218, t26653, t25346, t10109, t2053, t4272, t2047, t4142, t1492, t7084, t13042, t13053, t13065, t2054, t23250, t23254, t24318, t24321, t25168, t25339, t25343, t259);
        let (t26739, t26740) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1658::<F>(t26684, t26698, t26719, t26737, t870);
        let t26744 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1659::<F>(t2752, t7844);
    (t26700, t26703, t26708, t26713, t26722, t26728, t26729, t26732, t26734, t26739, t26740, t26744)
}
