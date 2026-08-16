//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2590/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2590(t45832: f64, t460: f64, t487: f64, t5219: f64, t5462: f64, t1209: f64, t21451: f64, t17191: f64, t3566: f64, t3781: f64, t5216: f64, t45618: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t59737 = t460 * t45832 * t487;
    let t59749 = t5219 * t5462;
    let t59788 = t1209 * t21451;
    let t59817 = t3566 * t17191;
    let t59854 = t5216 * t3781;
    let t59864 = t460 * t45618 * t487;
    (t59737, t59749, t59788, t59817, t59854, t59864)
}
