//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3175/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3175(t1256: f64, t24681: f64, t24671: f64, t21233: f64, t5391: f64, t21271: f64, t24846: f64, t3647: f64, t3670: f64, t5386: f64, t57548: f64, t57550: f64, t57606: f64, t70581: f64, t70583: f64, t70612: f64, t70616: f64, t77513: f64) -> f64 {
    let t83369 = t24681 * t1256;
    let t83371 = t24671 * t1256;
    let t83382 = t5391 * t21233;
    let t83384 = t57548 * t57606 * t77513 / 12.0_f64 - 7.0_f64 / 216.0_f64 * t57548 * t57550 * t77513 - 0.35400176935018568008e-1_f64 * t83369 - 0.22866142996303859718e-2_f64 * t83371 - 0.42874018118069736972e-3_f64 * t70581 - 0.15244095330869239812e-2_f64 * t70583 + 0.14291339372689912324e-2_f64 * t3647 * t24846 + 0.57165357490759649295e-3_f64 * t70612 - 0.85748036236139473944e-3_f64 * t70616 + 0.43445671692977333464e-1_f64 * t3670 * t21271 * t5386 - 0.2540682555144873302e-2_f64 * t83382;
    t83384
}
