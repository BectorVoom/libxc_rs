//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1465/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1465(t3103: f64, t4641: f64, t1040: f64, t4616: f64, t1612: f64, t3082: f64, t13969: f64, t4584: f64, t1041: f64, t4589: f64, t2960: f64, t4603: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14084 = t4641 * t3103 / 2304.0_f64;
    let t14085 = t4616 * t1040;
    let t14117 = t1612 * t3082;
    let t14134 = t13969 * t4584;
    let t14136 = t1041 * t14134 / 1728.0_f64;
    let t14137 = t13969 * t4589;
    let t14139 = 5.0_f64 / 10368.0_f64 * t1041 * t14137;
    let t14158 = t2960 * t4603 / 162.0_f64;
    (t14084, t14085, t14117, t14134, t14136, t14137, t14139, t14158)
}
