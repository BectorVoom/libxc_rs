//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 778/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk778(t274: f64, t4731: f64, t1684: f64, t45: f64, t1692: f64, t3005: f64, t1226: f64, t2919: f64, t2968: f64, t3013: f64, t3020: f64, t4612: f64, t4615: f64, t4618: f64, t4623: f64, t4658: f64, t4660: f64, t4701: f64, t4703: f64, t4706: f64, t4709: f64, t4712: f64, t4716: f64) -> (f64, f64, f64, f64, f64) {
    let t4732 = t4731 * t274;
    let t4735 = t45 * t1684;
    let t4740 = t3005 * t1692;
    let t4741 = t4740 * t1226;
    let t4758 = -0.1294625e1_f64 * t4658 + 0.258925e1_f64 * t4660 + t3013 + 0.10064166666666666667e0_f64 * t2919 + 0.10064166666666666667e0_f64 * t4612 - 0.20128333333333333333e0_f64 * t4615 + 0.60385e0_f64 * t4618 - 0.60385e0_f64 * t4623 + 0.82524375e-1_f64 * t4701 + 0.16504875e0_f64 * t4703 + t3020 + 0.5519e-1_f64 * t2968 + 0.5519e-1_f64 * t4706 - 0.27595e-1_f64 * t4709 + 0.16557e0_f64 * t4712 - 0.16557e0_f64 * t4716;
    (t4732, t4735, t4740, t4741, t4758)
}
