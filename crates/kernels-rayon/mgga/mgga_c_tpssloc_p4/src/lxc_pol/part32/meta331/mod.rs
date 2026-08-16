//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta331 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1363;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta331(t2535: f64, t3691: f64, t1372: f64, t3787: f64, t215: f64, t535: f64, t9569: f64, t1314: f64, t2559: f64, t1317: f64, t795: f64, t9580: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t12142, t12171, t12188, t12189, t12190, t12194) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1363(t2535, t3691, t1372, t3787, t215, t535, t9569, t1314, t2559, t1317, t795, t9580);
    (t12142, t12171, t12188, t12189, t12190, t12194)
}
