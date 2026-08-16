//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2396/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2396(t10750: f64, t10757: f64, t10820: f64, t10829: f64, t14337: f64, t14344: f64, t1581: f64, t2900: f64, t2907: f64, t42106: f64, t4472: f64, t49080: f64, t49082: f64, t49084: f64, t49086: f64, t49088: f64, t49090: f64, t49092: f64, t49095: f64, t49096: f64, t49099: f64, t49104: f64) -> f64 {
    let t49113 = -t49080 + t49082 - t49084 + t49086 - t49088 + t49090 - t49092 + t49095 - 0.35089341735807877242e1_f64 * t49096 * t2907 - 0.10389515463408878255e3_f64 * t49099 * t10829 + 0.35089341735807877242e1_f64 * t14337 * t10750 + 0.10254018858216406658e4_f64 * t49104 * t10757 + 0.5848223622634646207e0_f64 * t42106 * t1581 + 0.17544670867903938621e1_f64 * t10820 * t4472 + 0.17544670867903938621e1_f64 * t2900 * t14344;
    t49113
}
