//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1248/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1248(t35569: f64, t35573: f64, t35580: f64, t35585: f64, t35587: f64, t35601: f64, t35603: f64, t35616: f64, t37624: f64, t37625: f64, t37631: f64, t37632: f64, t40063: f64, t40068: f64, t40072: f64, t40076: f64, t40080: f64) -> f64 {
    let t41926 = 0.6289618457920830414e-2_f64 * t35569 + 0.42874018118069736972e-2_f64 * t40063 - 0.6289618457920830414e-2_f64 * t35573 + 0.25158473831683321656e-2_f64 * t35580 - 0.50316947663366643312e-2_f64 * t35585 + 0.17149607247227894789e-2_f64 * t35587 + t37624 + t37625 + 0.11321313224257494745e0_f64 * t35601 + t35603 - 0.17149607247227894789e-2_f64 * t40068 - 0.21437009059034868486e-3_f64 * t40072 - 0.31448092289604152069e-3_f64 * t40076 - 0.15724046144802076034e-2_f64 * t40080 + t37631 + t37632 - 0.31448092289604152069e-2_f64 * t35616;
    t41926
}
