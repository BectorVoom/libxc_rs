//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1665/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1665(t5308: f64, t9016: f64, t15868: f64, t2095: f64, t5161: f64, t7217: f64, t113: f64, t19456: f64, t1983: f64, t2040: f64, t2096: f64, t22574: f64, t24987: f64, t24995: f64, t26161: f64, t26559: f64, t26870: f64, t26872: f64, t4028: f64, t6876: f64, t7050: f64, t7057: f64, t7171: f64, t7220: f64, t7685: f64, t7904: f64, t7943: f64) -> (f64, f64, f64, f64) {
    let t26875 = t9016 * t5308;
    let t26878 = t2095 * t15868;
    let t26880 = t7217 * t5161;
    let t26895 = -t113 * t26870 - 2.0_f64 * t19456 * t2040 - t1983 * t26878 - t1983 * t26880 + t2096 * t24987 - 3.0_f64 * t22574 * t26872 + 6.0_f64 * t24995 * t26875 + 2.0_f64 * t26161 * t26559 - 2.0_f64 * t4028 * t7050 - 2.0_f64 * t4028 * t7057 + 3.0_f64 * t6876 * t7904 - t6876 * t7943 + 3.0_f64 * t7171 * t7685 - t7220 * t7685;
    (t26875, t26878, t26880, t26895)
}
