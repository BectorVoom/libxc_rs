//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta500 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1816;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta500(t1060: f64, t25499: f64, t4688: f64, t6800: f64, t6799: f64, t23665: f64, t7611: f64, t1936: f64, t362: f64) -> (f64, f64, f64, f64, f64) {
        let (t25500, t25502, t25503, t25508, t25510) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1816(t1060, t25499, t4688, t6800, t6799, t23665, t7611, t1936, t362);
    (t25500, t25502, t25503, t25508, t25510)
}
