//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1276/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1276(t1394: f64, t5667: f64, t98470: f64, t20887: f64, t4153: f64, t7923: f64, t1615: f64, t6176: f64, t7509: f64, t94862: f64, t27614: f64, t7492: f64) -> (f64, f64, f64, f64) {
    let t101868 = t1394 * t98470 * t5667;
    let t101871 = t4153 * t7923 * t20887;
    let t101875 = t6176 * t94862 * t7509 * t1615;
    let t101884 = t6176 * t27614 * t7492 * t1615;
    (t101868, t101871, t101875, t101884)
}
