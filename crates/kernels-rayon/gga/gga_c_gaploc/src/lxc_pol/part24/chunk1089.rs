//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1089/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1089(t15667: f64, t27847: f64, t15665: f64, t15672: f64, t27846: f64, t4066: f64, t92: f64, t27842: f64, t5345: f64, t5348: f64, t1695: f64, t3220: f64) -> (f64, f64, f64, f64) {
    let t27848 = t27847 * t15667;
    let t27853 = t15672 * t4066 * t27846 * t15665 * t92;
    let t27856 = t5345 * t27842 * t5348;
    let t27858 = t3220 * t1695;
    (t27848, t27853, t27856, t27858)
}
