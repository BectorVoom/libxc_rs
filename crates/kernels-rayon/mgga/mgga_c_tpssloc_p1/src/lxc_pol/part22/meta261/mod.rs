//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta261 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1401;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1402;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1403;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta261(t11153: f64, t3439: f64, t11147: f64, t11545: f64, t3247: f64, t415: f64, t61: f64, t121: f64, t3584: f64, t1229: f64, t676: f64, t1090: f64, t248: f64, t1227: f64, t486: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11759, t11764, t11778) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1401(t11153, t3439, t11147, t11545, t3247, t415);
        let (t11779, t11784, t11789) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1402(t11778, t61, t121, t3584, t1229, t676);
        let (t11791, t11792, t11818) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1403(t1090, t11789, t248, t1227, t486, t676);
    (t11759, t11764, t11778, t11779, t11784, t11789, t11791, t11792, t11818)
}
