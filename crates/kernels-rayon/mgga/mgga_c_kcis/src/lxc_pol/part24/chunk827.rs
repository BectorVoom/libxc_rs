//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 827/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk827(t14316: f64, t4633: f64, t4606: f64, t4670: f64, t3293: f64, t1035: f64, t6316: f64, t934: f64, t1045: f64, t6317: f64, t4642: f64, t1728: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t18539 = t14316 * t4633;
    let t18542 = t4606 * t4670;
    let t18543 = t3293 * t18542;
    let t18546 = t1035 * t6316;
    let t18547 = t18546 * t934;
    let t18548 = t3293 * t18547;
    let t18551 = t6317 * t1045;
    let t18552 = t4642 * t18551;
    let t18555 = t1728 * t4670;
    (t18539, t18542, t18543, t18547, t18548, t18551, t18552, t18555)
}
