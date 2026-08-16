//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 751/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk751(t10012: f64, t10627: f64, t10892: f64, t1980: f64, t1858: f64, t3431: f64, t1890: f64, t3487: f64, t107: f64, t10809: f64, t787: f64, t32214: f64, t739: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33148 = t10012 * t10627;
    let t33206 = t1980 * t10892;
    let t33232 = t1858 * t3431;
    let t33289 = t1890 * t3487;
    let t33294 = t787 * t10809 * t107;
    let t33300 = t739 * t32214;
    (t33148, t33206, t33232, t33289, t33294, t33300)
}
