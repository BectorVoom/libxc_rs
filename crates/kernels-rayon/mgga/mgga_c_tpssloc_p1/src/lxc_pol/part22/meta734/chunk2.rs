//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2412/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2412(t14473: f64, t5808: f64, t5790: f64, t950: f64, t4475: f64, t49532: f64, t4472: f64, t5811: f64, t959: f64, t1589: f64, t60848: f64, t68767: f64, t68769: f64, t68771: f64, t68773: f64, t68775: f64, t68883: f64, t68885: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t68887 = 0.17544670867903938621e1_f64 * t14473 * t5808;
    let t68888 = t5790 * t950;
    let t68891 = 0.31168546390226634766e3_f64 * t49532 * t4475 * t68888;
    let t68894 = 0.10526802520742363173e2_f64 * t959 * t5811 * t4472;
    let t68896 = 0.17544670867903938621e1_f64 * t60848 * t1589;
    let t68897 = t68767 + t68769 + t68771 - t68773 + t68775 + t68883 + t68885 - t68887 + t68891 - t68894 - t68896;
    (t68887, t68888, t68891, t68894, t68896, t68897)
}
