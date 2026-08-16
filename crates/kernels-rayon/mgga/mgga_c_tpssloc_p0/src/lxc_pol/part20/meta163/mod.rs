//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta163 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1025;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1026;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1027;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1028;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1029;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1030;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1031;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1032;
use chunk8::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1033;
use chunk9::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1034;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta163(t3850: f64, t550: f64, t1343: f64, t820: f64, t3791: f64, t248: f64, t2691: f64, t557: f64, t555: f64, t1361: f64, t835: f64, t1336: f64, t1369: f64, t1995: f64, t241: f64, t67: f64, t3734: f64, t1367: f64, t3719: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3851 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1025(t3850, t550);
        let t3853 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1026(t1343, t3851, t820);
        let t3856 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1027(t3791, t550);
        let t3858 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1028(t1343, t3856, t820);
        let t3862 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1029(t248, t2691, t557);
        let (t3864, t3865) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1030(t3862, t555, t1361, t835);
        let t3866 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1031(t1336, t3865);
        let (t3867, t3870) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1032(t1369, t3866, t1995, t241, t67);
        let t3872 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1033(t3734, t3870, t820);
        let t3876 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1034(t1367, t3719, t820);
    (t3851, t3853, t3856, t3858, t3862, t3864, t3865, t3866, t3867, t3870, t3872, t3876)
}
