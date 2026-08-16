//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1118/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1118(t33153: f64, t1458: f64, t31224: f64, t33124: f64, t33142: f64, t33144: f64, t33146: f64, t33148: f64, t33150: f64, t33152: f64, t8446: f64, t5161: f64, t8489: f64) -> (f64, f64, f64) {
    let t33154 = 2.0_f64 * t33153;
    let t33155 = 2.0_f64 * t1458 * t31224 + t33124 + 4.0_f64 * t33142 + 4.0_f64 * t33144 + 4.0_f64 * t33146 + t33148 + t33150 + t33152 + t33154 + t8446;
    let t33157 = t8489 * t5161;
    (t33154, t33155, t33157)
}
