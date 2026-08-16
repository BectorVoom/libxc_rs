//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1223/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1223(t12804: f64, t3720: f64, t1209: f64, t3781: f64, t5330: f64, t3153: f64, t3601: f64) -> (f64, f64, f64, f64) {
    let t12805 = t3720 * t12804;
    let t12808 = t1209 * t3781;
    let t12809 = t12808 * t5330;
    let t12810 = t3601 * t3153;
    (t12805, t12808, t12809, t12810)
}
