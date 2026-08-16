//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 747/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk747(t31747: f64, t6508: f64, t2787: f64, t6509: f64, t123: f64, t25760: f64, t1352: f64, t3339: f64, t10281: f64, t501: f64, t1853: f64, t3432: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31748 = t6508 * t31747;
    let t31769 = t2787 * t6509;
    let t31903 = t25760 * t123;
    let t32067 = t3339 * t1352;
    let t32100 = t10281 * t501;
    let t32112 = t3432 * t1853;
    (t31748, t31769, t31903, t32067, t32100, t32112)
}
