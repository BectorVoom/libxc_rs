//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta287 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1195;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1196;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta287(t2374: f64, t9888: f64, t2509: f64, t745: f64, t9843: f64, t761: f64, t152: f64, t31: f64, t2448: f64, t67: f64, t758: f64, t2368: f64, t2505: f64, t2250: f64, t751: f64, t707: f64, t2447: f64, t706: f64, t746: f64, t9490: f64, t2531: f64, t2535: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9890, t9892, t9894, t9897, t9902, t9905) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1195(t2374, t9888, t2509, t745, t9843, t761, t152, t31, t2448, t67, t758, t2368, t2505);
        let (t9907, t9910, t9912, t9919, t9921, t9922) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1196(t761, t9905, t2250, t751, t707, t2447, t706, t2509, t746, t9490, t2531, t2535);
    (t9890, t9892, t9894, t9897, t9902, t9905, t9907, t9910, t9912, t9919, t9921, t9922)
}
