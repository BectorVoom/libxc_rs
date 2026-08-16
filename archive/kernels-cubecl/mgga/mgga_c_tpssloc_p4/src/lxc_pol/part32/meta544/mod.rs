//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta544 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1892;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1893;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1894;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta544<F: Float>(t27710: F, t7325: F, t2132: F, t25588: F, t2121: F, t2136: F, t24650: F, t24747: F, t24752: F, t24754: F, t27681: F, t27684: F, t27687: F, t27692: F, t27697: F, t27701: F, t27704: F, t4989: F, t7321: F, t7326: F, t7331: F, t7345: F, t8040: F, t27602: F, t27648: F, t27679: F, t493: F, t1734: F, t7348: F, t1246: F, t24574: F, t8070: F, t2147: F, t5052: F, t462: F, t1170: F, t8077: F, t1201: F, t1244: F, t1729: F, t2152: F, t24856: F, t27572: F, t27574: F, t470: F, t4964: F, t7283: F, t7382: F, t7389: F, t7999: F, t8085: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t27711, t27714, t27719) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1892::<F>(t27710, t7325, t2132, t25588, t2121, t2136, t24650, t24747, t24752, t24754, t27681, t27684, t27687, t27692, t27697, t27701, t27704, t4989, t7321, t7326, t7331, t7345, t8040);
        let (t27721, t27722, t27725, t27728, t27732) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1893::<F>(t27602, t27648, t27679, t27719, t493, t1734, t7348, t1246, t24574, t8070, t2147, t5052);
        let (t27733, t27736, t27737, t27739) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1894::<F>(t27732, t462, t1170, t8077, t2121, t1201, t1244, t1729, t2152, t24856, t27572, t27574, t27722, t27725, t27728, t470, t4964, t7283, t7382, t7389, t7999, t8085);
    (t27711, t27714, t27721, t27722, t27725, t27728, t27732, t27733, t27736, t27737, t27739)
}
