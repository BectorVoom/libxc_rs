//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1605/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1605<F: Float>(t61090: F, t76947: F, t76949: F, t76951: F, t49897: F, t18259: F, t23216: F, t1469: F, t4401: F, t77042: F, t18263: F, t5999: F) -> (F, F, F, F, F, F, F, F) {
    let t87303 = F::cast_from(24.0_f64) * t61090;
    let t87304 = F::cast_from(144.0_f64) * t76947;
    let t87305 = F::cast_from(48.0_f64) * t76949;
    let t87306 = F::cast_from(4.0_f64) * t76951;
    let t87307 = F::cast_from(0.23392894490538584828e1_f64) * t49897;
    let t87309 = F::cast_from(144.0_f64) * t18259 * t23216;
    let t87312 = F::cast_from(48.0_f64) * t4401 * t77042 * t1469;
    let t87314 = F::cast_from(24.0_f64) * t18263 * t5999;
    (t87303, t87304, t87305, t87306, t87307, t87309, t87312, t87314)
}
