//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta534 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2194;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2195;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta534(t18047: f64, t383: f64, t4684: f64, t5932: f64, t3188: f64, t4649: f64, t1629: f64, t4673: f64, t1625: f64, t1060: f64, t1022: f64, t5914: f64, t17959: f64, t381: f64, t1003: f64, t1058: f64, t1063: f64, t14608: f64, t1610: f64, t1632: f64, t17876: f64, t3180: f64, t3186: f64, t3200: f64, t353: f64, t384: f64, t4615: f64, t4669: f64, t4678: f64, t4681: f64, t4685: f64, t4689: f64, t4691: f64, t5903: f64, t5933: f64, t5941: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18129, t18131, t18138, t18139, t18142, t18150, t18151, t18154) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2194(t18047, t383, t4684, t5932, t3188, t4649, t1629, t4673, t1625, t1060, t1022, t5914);
        let (t18155, t18161, t18162, t18164) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2195(t1060, t18154, t17959, t381, t1003, t1058, t1063, t14608, t1610, t1632, t17876, t18129, t18131, t18139, t18142, t18151, t3180, t3186, t3200, t353, t384, t4615, t4669, t4678, t4681, t4685, t4689, t4691, t5903, t5933, t5941);
    (t18129, t18131, t18138, t18139, t18142, t18150, t18151, t18155, t18161, t18162, t18164)
}
