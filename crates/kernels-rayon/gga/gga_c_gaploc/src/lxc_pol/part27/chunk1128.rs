//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1128/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1128(t29874: f64, t9075: f64, t1365: f64, t20540: f64, t23983: f64, t484: f64, t9087: f64, t145: f64, t27835: f64, t459: f64, t1242: f64, t27839: f64) -> (f64, f64, f64, f64, f64) {
    let t29876 = 0.47425011059460249332e-2_f64 * t29874 * t9075;
    let t29879 = 0.47425011059460249332e-2_f64 * t23983 * t1365 * t20540;
    let t29892 = 0.63233348079280332442e-2_f64 * t484 * t9087;
    let t29896 = t27835 * t145 * t459;
    let t29898 = t27839 * t1242;
    (t29876, t29879, t29892, t29896, t29898)
}
