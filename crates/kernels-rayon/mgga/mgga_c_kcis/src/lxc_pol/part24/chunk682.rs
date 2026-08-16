//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 682/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk682(t1657: f64, t441: f64, t1876: f64, t235: f64, t1658: f64, t2209: f64, t1250: f64, t4981: f64) -> (f64, f64, f64, f64) {
    let t8021 = t1657 * t441;
    let t8024 = t235 * t1876;
    let t8027 = t1658 * t2209;
    let t8030 = t4981 * t1250;
    (t8021, t8024, t8027, t8030)
}
