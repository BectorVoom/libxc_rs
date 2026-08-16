//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta534 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2194;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2195;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta534<F: Float>(t18047: F, t383: F, t4684: F, t5932: F, t3188: F, t4649: F, t1629: F, t4673: F, t1625: F, t1060: F, t1022: F, t5914: F, t17959: F, t381: F, t1003: F, t1058: F, t1063: F, t14608: F, t1610: F, t1632: F, t17876: F, t3180: F, t3186: F, t3200: F, t353: F, t384: F, t4615: F, t4669: F, t4678: F, t4681: F, t4685: F, t4689: F, t4691: F, t5903: F, t5933: F, t5941: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t18129, t18131, t18138, t18139, t18142, t18150, t18151, t18154) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2194::<F>(t18047, t383, t4684, t5932, t3188, t4649, t1629, t4673, t1625, t1060, t1022, t5914);
        let (t18155, t18161, t18162, t18164) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2195::<F>(t1060, t18154, t17959, t381, t1003, t1058, t1063, t14608, t1610, t1632, t17876, t18129, t18131, t18139, t18142, t18151, t3180, t3186, t3200, t353, t384, t4615, t4669, t4678, t4681, t4685, t4689, t4691, t5903, t5933, t5941);
    (t18129, t18131, t18138, t18139, t18142, t18150, t18151, t18155, t18161, t18162, t18164)
}
