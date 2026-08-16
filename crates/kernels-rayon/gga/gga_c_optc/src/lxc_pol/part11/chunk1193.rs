//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1193/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1193(t1179: f64, t18102: f64, t2586: f64, t12635: f64, t15873: f64, t17855: f64, t438: f64, t17921: f64, t4457: f64, t45343: f64, t3103: f64, t44014: f64, t5324: f64) -> (f64, f64, f64, f64, f64) {
    let t54797 = t1179 * t2586 * t18102;
    let t54799 = t12635 * t15873;
    let t54837 = t17855 * t438;
    let t54843 = t4457 * t45343 * t17921;
    let t54846 = t3103 * t44014 * t5324;
    (t54797, t54799, t54837, t54843, t54846)
}
