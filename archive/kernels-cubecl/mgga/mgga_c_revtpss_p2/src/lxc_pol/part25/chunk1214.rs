//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1214/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1214<F: Float>(t10406: F, t76: F, t38: F, t45955: F, t2242: F, t2251: F, t2247: F, t25138: F, t1923: F, t1926: F, t1928: F, t25102: F, t25106: F, t25110: F, t25139: F, t25143: F, t25146: F, t25147: F, t25150: F, t6954: F, t6960: F, t6973: F, t6974: F, t6977: F, t6978: F) -> F {
    let t92628 = t76 * t10406;
    let t92632 = t45955 * t38;
    let t92639 = t2242 * t2251;
    let t92644 = t2247 * t38 * t25138;
    let t92649 = -t1923 * t25139 * t6977 / F::cast_from(2.0_f64) - t6954 * t25143 - t1923 * t6973 * t25146 / F::cast_from(2.0_f64) - t6954 * t25147 / F::cast_from(2.0_f64) - t1923 * t1926 * t92628 / F::cast_from(6.0_f64) - t92632 * t1928 / F::cast_from(6.0_f64) - t25150 * t6974 / F::cast_from(2.0_f64) - t25150 * t6978 / F::cast_from(2.0_f64) + t92639 * t1928 + F::cast_from(2.0_f64) * t25102 * t6978 + F::cast_from(5.0_f64) / F::cast_from(2.0_f64) * t92644 * t6960 + F::cast_from(5.0_f64) * t25106 * t25110;
    t92649
}
