//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2502/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2502(t10985: f64, t15017: f64, t15045: f64, t2435: f64, t15048: f64, t2471: f64, t15008: f64, t2439: f64, t4469: f64, t780: f64, t785: f64, t213: f64, t252: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t50214 = t15017 * t10985;
    let t50218 = t2435 * t15045;
    let t50219 = 0.21951497276451705329e-1_f64 * t50218;
    let t50220 = t15048 * t2471;
    let t50221 = 0.39029762157531132076e-1_f64 * t50220;
    let t50222 = t2435 * t15008;
    let t50223 = 0.21951497276451705329e-1_f64 * t50222;
    let t50236 = t2439 * t785 * t4469 * t780;
    let t50240 = t213 * t252;
    (t50214, t50219, t50221, t50223, t50236, t50240)
}
