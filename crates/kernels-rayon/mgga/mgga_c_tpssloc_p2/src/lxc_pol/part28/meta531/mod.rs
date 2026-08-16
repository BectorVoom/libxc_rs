//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta531 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1785;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta531(t154: f64, t9533: f64, t131: f64, t3748: f64, t2009: f64, t9537: f64, t22642: f64, t22690: f64, t22881: f64, t2690: f64, t22691: f64, t1887: f64, t22797: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t81142, t81144, t81146, t81149, t81151, t81152, t81153, t81159) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1785(t154, t9533, t131, t3748, t2009, t9537, t22642, t22690, t22881, t2690, t22691, t1887, t22797);
    (t81142, t81144, t81146, t81149, t81151, t81152, t81153, t81159)
}
