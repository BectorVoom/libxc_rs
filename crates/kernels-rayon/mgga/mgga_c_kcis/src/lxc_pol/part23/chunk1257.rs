//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1257/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1257(t556: f64, t94424: f64, t15883: f64, t5661: f64, t18210: f64, t28402: f64, t7898: f64, t1983: f64, t303: f64, t4137: f64, t1464: f64, t15955: f64, t27387: f64, t3722: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98530 = t94424 * t556;
    let t98532 = t5661 * t98530 * t15883;
    let t98537 = t18210 * t28402;
    let t98538 = t7898 * t98537;
    let t98543 = t303 * t1983 * t4137;
    let t98553 = t1464 * t27387 * t15955 * t3722;
    (t98530, t98532, t98537, t98538, t98543, t98553)
}
