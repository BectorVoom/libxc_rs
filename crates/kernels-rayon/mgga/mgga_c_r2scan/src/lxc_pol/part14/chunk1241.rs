//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1241/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1241(t40821: f64, t40840: f64, t40844: f64, t37029: f64, t37041: f64, t37063: f64, t37066: f64, t38792: f64, t38808: f64, t40817: f64, t40825: f64, t40828: f64, t40830: f64, t40833: f64, t40835: f64, t40837: f64, t40842: f64, t40848: f64) -> f64 {
    let t41877 = 8.0_f64 * t40821;
    let t41885 = 4.0_f64 / 3.0_f64 * t40840;
    let t41887 = 4.0_f64 / 3.0_f64 * t40844;
    let t41890 = 6.0_f64 * t40817 + t41877 - 3.0_f64 * t40825 - 3.0_f64 / 2.0_f64 * t40828 + t40830 / 4.0_f64 + 4.0_f64 / 3.0_f64 * t37029 - t40833 - t40835 / 2.0_f64 - t40837 / 4.0_f64 + 44.0_f64 / 9.0_f64 * t37041 + t38792 - t41885 + 3.0_f64 / 2.0_f64 * t40842 + t41887 - 44.0_f64 / 9.0_f64 * t37066 + t38808 + 2.0_f64 / 3.0_f64 * t37063 + t40848;
    t41890
}
