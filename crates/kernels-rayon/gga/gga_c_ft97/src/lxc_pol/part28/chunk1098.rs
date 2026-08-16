//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1098/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1098(t1058: f64, t7339: f64, t1984: f64, t34918: f64, t1349: f64, t138445: f64, t138476: f64, t138511: f64, t1969: f64, t23925: f64, t26800: f64, t26809: f64, t26811: f64, t26817: f64, t28: f64, t3051: f64, t3052: f64, t32717: f64, t32719: f64, t32722: f64, t32879: f64, t32881: f64, t3450: f64, t35010: f64, t379: f64, t5772: f64, t5779: f64, t7308: f64, t9073: f64, t925: f64, t9432: f64) -> (f64, f64) {
    let t147112 = t7339 * t1058;
    let t147122 = t1984 * t34918;
    let t147132 = -t5772 * t1969 * t138511 * t925 / 18.0_f64 + 2.0_f64 / 9.0_f64 * t26809 * t9073 * t32717 * t3052 - t5772 * t138445 * t26800 / 3.0_f64 - t7308 * t3051 * t26811 / 9.0_f64 + t26817 * t32719 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t26809 * t1969 * t32879 * t3052 - t5772 * t1969 * t147112 * t379 / 18.0_f64 - t26817 * t32881 / 9.0_f64 + t5772 * t9432 * t32722 * t3450 - t1349 * t28 * t147122 * t5779 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t1349 * t28 * t23925 * t35010 - t138476 / 18.0_f64;
    (t147122, t147132)
}
