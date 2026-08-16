//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta287 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1587;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1588;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta287(t10422: f64, t3072: f64, t3070: f64, t1005: f64, t3082: f64, t1036: f64, t3094: f64, t3089: f64, t248: f64, t2780: f64, t3051: f64, t1041: f64, t121: f64, t3061: f64, t2771: f64, t1008: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10423, t10424, t10436, t10441, t10449, t10454, t10455) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1587(t10422, t3072, t3070, t1005, t3082, t1036, t3094, t3089, t248, t2780, t3051, t1041);
        let (t10457, t10459, t10460, t10468, t10469) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1588(t121, t3061, t248, t2771, t1041, t1008);
    (t10423, t10424, t10436, t10441, t10449, t10454, t10455, t10457, t10459, t10460, t10468, t10469)
}
