//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta560 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1963;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1964;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1965;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1966;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta560(t4724: f64, t4899: f64, t1210: f64, t8039: f64, t24721: f64, t1714: f64, t2133: f64, t2132: f64, t6739: f64, t8026: f64, t7325: f64, t25588: f64, t2121: f64, t2136: f64, t24650: f64, t24747: f64, t24752: f64, t24754: f64, t27681: f64, t27684: f64, t27687: f64, t27692: f64, t4989: f64, t7321: f64, t7326: f64, t7331: f64, t7345: f64, t8040: f64, t27602: f64, t27648: f64, t27679: f64, t493: f64, t1734: f64, t7348: f64, t1246: f64, t24574: f64, t8070: f64, t2147: f64, t5052: f64, t462: f64, t1170: f64, t8077: f64, t1201: f64, t1244: f64, t1729: f64, t2152: f64, t24856: f64, t27572: f64, t27574: f64, t470: f64, t4964: f64, t7283: f64, t7382: f64, t7389: f64, t7999: f64, t8085: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27697, t27700, t27701, t27704, t27710, t27711, t27714) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1963(t4724, t4899, t1210, t8039, t24721, t1714, t2133, t2132, t6739, t8026, t7325, t25588);
        let t27719 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1964(t2121, t2136, t24650, t24747, t24752, t24754, t27681, t27684, t27687, t27692, t27697, t27701, t27704, t27711, t27714, t4989, t7321, t7326, t7331, t7345, t8040);
        let (t27721, t27722, t27724, t27725, t27728, t27732) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1965(t27602, t27648, t27679, t27719, t493, t1734, t7348, t1246, t24574, t8070, t2147, t5052);
        let t27739 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1966(t27732, t462, t1170, t8077, t2121, t1201, t1244, t1729, t2152, t24856, t27572, t27574, t27722, t27725, t27728, t470, t4964, t7283, t7382, t7389, t7999, t8085);
    (t27700, t27704, t27710, t27711, t27714, t27721, t27722, t27724, t27725, t27732, t27739)
}
