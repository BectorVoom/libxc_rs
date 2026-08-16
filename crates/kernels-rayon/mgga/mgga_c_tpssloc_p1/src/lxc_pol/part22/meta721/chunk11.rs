//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2355/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2355(t10054: f64, t1499: f64, t1525: f64, t16754: f64, t16805: f64, t17023: f64, t20853: f64, t20854: f64, t20857: f64, t20858: f64, t20861: f64, t20862: f64, t20937: f64, t2617: f64, t2732: f64, t40917: f64, t4166: f64, t4298: f64, t5575: f64, t812: f64, t863: f64) -> f64 {
    let t68299 = 6.0_f64 * t10054 * t20861 * t812 - t20853 * t2732 * t812 - 6.0_f64 * t20857 * t40917 * t812 + 3.0_f64 * t1499 * t17023 + 3.0_f64 * t1525 * t16805 - 3.0_f64 * t16754 * t4166 - t20854 * t2617 - 6.0_f64 * t20858 * t2617 + 6.0_f64 * t20862 * t2617 + t20937 * t863 + 3.0_f64 * t4298 * t5575;
    t68299
}
