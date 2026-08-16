//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2085/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2085<F: Float>(t12167: F, t99984: F, t12078: F, t25516: F, t4954: F, t15752: F, t27498: F, t15734: F, t25522: F, t15816: F, t7121: F, t15794: F, t25580: F) -> (F, F, F, F, F, F, F) {
    let t100138 = t12167 * t99984;
    let t100141 = t12078 * t99984;
    let t100146 = t4954 * t25516;
    let t100160 = F::cast_from(0.57165357490759649296e-3_f64) * t27498 * t15752;
    let t100166 = t25522 * t15734;
    let t100168 = t15816 * t7121;
    let t100186 = F::cast_from(0.57165357490759649296e-3_f64) * t25580 * t15794;
    (t100138, t100141, t100146, t100160, t100166, t100168, t100186)
}
