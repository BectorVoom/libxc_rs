//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2072/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2072<F: Float>(t25375: F, t99125: F, t25387: F, t27182: F, t686: F, t72: F, t2435: F, t27334: F, t10867: F, t1949: F, t14485: F, t25399: F) -> (F, F, F, F, F, F, F) {
    let t99127 = F::cast_from(0.28912093960683998208e-1_f64) * t25375 * t99125;
    let t99147 = F::cast_from(0.51405703062096148812e-1_f64) * t25387 * t99125;
    let t99161 = t27182 * t72 * t686;
    let t99163 = F::cast_from(0.51405703062096148812e-1_f64) * t25387 * t99161;
    let t99166 = t2435 * t27334;
    let t99174 = t10867 * t1949;
    let t99186 = t25399 * t14485;
    (t99127, t99147, t99161, t99163, t99166, t99174, t99186)
}
