//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 569/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk569(t3295: f64, t7354: f64, t2684: f64, t1: f64, t9636: f64, t787: f64, t9755: f64, t2365: f64, t7069: f64, t7390: f64, t531: f64, t9689: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10029 = t7354 * t3295;
    let t10030 = t2684 * t10029;
    let t10031 = 0.51123901271894332901e0_f64 * t10030;
    let t10032 = t9636 * t1;
    let t10033 = t787 * t10032;
    let t10036 = t9755 * t1;
    let t10037 = t787 * t10036;
    let t10040 = t2365 * t7069;
    let t10042 = 0.29792074959875355558e-1_f64 * t7390 * t10040;
    let t10043 = t531 * t9689;
    (t10030, t10031, t10032, t10033, t10036, t10037, t10040, t10042, t10043)
}
