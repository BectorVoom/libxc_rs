//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta290 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1061;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1062;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1063;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta290(t12364: f64, t1336: f64, t1354: f64, t120: f64, t3791: f64, t1307: f64, t3792: f64, t3805: f64, t1328: f64, t210: f64, t3719: f64, t12178: f64, t1343: f64, t820: f64, t3788: f64, t835: f64, t3795: f64, t3799: f64, t3853: f64, t12353: f64, t12356: f64, t12358: f64, t12361: f64, t1341: f64, t1363: f64, t3733: f64, t3778: f64, t3858: f64, t5246: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12365, t12366, t12368) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1061(t12364, t1336, t1354, t120, t3791);
        let (t12369, t12371, t12375, t12379, t12384) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1062(t1307, t3792, t12368, t3805, t1328, t210, t3719, t12178, t1343, t820, t3788, t835);
        let (t12385, t12390) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1063(t12384, t1336, t3795, t3799, t3853, t12353, t12356, t12358, t12361, t12366, t12371, t12375, t12379, t1341, t1363, t3733, t3778, t3858, t5246);
    (t12365, t12368, t12369, t12371, t12375, t12379, t12384, t12385, t12390)
}
