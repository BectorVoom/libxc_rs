//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 958/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk958<F: Float>(t644: F, t77: F, t7705: F, t1497: F, t1927: F, t1926: F, t1470: F, t2247: F, t1928: F, t25099: F, t25157: F, t25162: F, t25164: F, t28116: F, t28119: F, t28127: F, t28133: F, t28138: F, t28141: F, t6958: F, t6960: F, t6963: F, t6974: F, t6978: F, t7706: F, t7709: F, t7716: F, t7720: F) -> (F, F, F, F) {
    let t28147 = t77 * t7705 * t644;
    let t28150 = t1927 * t1497;
    let t28151 = t1926 * t28150;
    let t28154 = t2247 * t1470;
    let t28157 = t28116 * t1928 / F::cast_from(3.0_f64) + t28119 * t1928 / F::cast_from(3.0_f64) + t7709 * t6974 / F::cast_from(3.0_f64) + t7709 * t6978 / F::cast_from(3.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t28127 * t6960 + t6963 * t7716 / F::cast_from(3.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t6958 * t28133 + t6963 * t7720 / F::cast_from(3.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t28138 * t6960 + t28141 * t1928 / F::cast_from(3.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t25099 * t7706 - F::cast_from(5.0_f64) * t25157 * t28147 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t25162 * t28151 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t28154 * t25164;
    (t28147, t28150, t28154, t28157)
}
