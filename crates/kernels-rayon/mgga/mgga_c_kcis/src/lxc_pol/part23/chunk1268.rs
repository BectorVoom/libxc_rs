//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1268/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1268(t15866: f64, t4160: f64, t98034: f64, t2002: f64, t303: f64, t94528: f64, t1498: f64, t5871: f64, t1983: f64, t3723: f64, t3245: f64, t8168: f64) -> (f64, f64, f64, f64, f64) {
    let t98706 = t4160 * t98034 * t15866;
    let t98709 = t303 * t94528 * t2002;
    let t98712 = t303 * t5871 * t1498;
    let t98715 = t303 * t1983 * t3723;
    let t98719 = t3245 * t8168;
    (t98706, t98709, t98712, t98715, t98719)
}
