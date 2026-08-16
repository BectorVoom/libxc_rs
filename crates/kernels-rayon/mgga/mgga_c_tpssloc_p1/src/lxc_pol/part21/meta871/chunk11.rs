//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3211/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3211(t1254: f64, t1256: f64, t15834: f64, t193: f64, t336: f64, t4700: f64, t5095: f64, t63714: f64, t63717: f64, t63720: f64, t63722: f64, t63725: f64, t63729: f64, t64548: f64, t64558: f64, t64562: f64, t64564: f64, t64566: f64, t64602: f64, t65206: f64, t66842: f64, t66879: f64) -> f64 {
    let t66885 = -2.0_f64 * t4700 * t64548 * t1254 - 2.0_f64 * t4700 * t5095 * t15834 - t64558 + t64562 - t64564 - t64566 + t63714 + t63717 + t63720 + t63722 + t63725 + t63729 + t193 * t336 * (t64602 + t65206 + t66842 + t66879) * t1256;
    t66885
}
