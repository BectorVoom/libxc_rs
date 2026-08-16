//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta363 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1284;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta363(t12339: f64, t1831: f64, t3866: f64, t5314: f64, t3865: f64, t5234: f64, t1369: f64, t12189: f64, t1811: f64, t1358: f64, t5231: f64, t1815: f64, t3862: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t16325, t16331, t16336, t16338, t16341, t16346, t16350) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1284(t12339, t1831, t3866, t5314, t3865, t5234, t1369, t12189, t1811, t1358, t5231, t1815, t3862);
    (t16325, t16331, t16336, t16338, t16341, t16346, t16350)
}
