//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 867/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk867(t4791: f64, t4794: f64, t4798: f64, t4806: f64, t4992: f64, t5999: f64, t6002: f64, t6961: f64, t6975: f64, t7009: f64, t7865: f64, t2823: f64, t6001: f64) -> (f64, f64) {
    let t7869 = -t6961 + 0.285764e-1_f64 * t7865 - t4791 + t4794 + t4798 - t4806 + t6975 + t4992 - 0.1350520664e0_f64 * t5999 - 0.1350520664e0_f64 * t6002 - t7009;
    let t7870 = t2823 * t6001;
    (t7869, t7870)
}
