//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta440 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1785;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1786;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta440(t12050: f64, t12091: f64, t12044: f64, t12048: f64, t12057: f64, t12059: f64, t12087: f64, t12094: f64, t15898: f64, t15911: f64, t15916: f64, t15917: f64, t15923: f64, t19599: f64, t9780: f64, t9789: f64, t172: f64, t6320: f64, t763: f64, t15972: f64, t12097: f64, t12106: f64, t12111: f64, t12103: f64, t12105: f64, t12109: f64, t12114: f64, t12116: f64, t12118: f64, t15976: f64, t9793: f64, t9797: f64, t9820: f64, t9824: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19677, t19678, t19679) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1785(t12050, t12091, t12044, t12048, t12057, t12059, t12087, t12094, t15898, t15911, t15916, t15917, t15923, t19599, t9780, t9789);
        let (t19681, t19682, t19683, t19684, t19685, t19686, t19687, t19688) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1786(t172, t6320, t763, t15972, t12097, t12106, t12111, t12103, t12105, t12109, t12114, t12116, t12118, t15976, t9793, t9797, t9820, t9824);
    (t19677, t19678, t19679, t19681, t19682, t19683, t19684, t19685, t19686, t19687, t19688)
}
