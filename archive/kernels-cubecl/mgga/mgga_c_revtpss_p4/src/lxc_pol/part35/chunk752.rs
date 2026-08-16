//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 752/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk752<F: Float>(t760: F, t9318: F, t162: F, t9544: F, t158: F, t755: F, t9586: F, t2629: F, t9863: F, t9866: F, t9575: F, t9572: F) -> (F, F, F, F, F, F, F) {
    let t10554 = F::cast_from(0.35089341735807877242e1_f64) * t760 * t9318;
    let t10565 = t162 * t9544;
    let t10566 = t158 * t10565;
    let t10568 = F::cast_from(0.56968947174242584612e-3_f64) * t755 * t9586;
    let t10577 = F::cast_from(0.16265371950452609763e-1_f64) * t2629 * t9863;
    let t10582 = F::cast_from(0.48159733137676571078e0_f64) * t2629 * t9866;
    let t10584 = F::cast_from(0.21687162600603479684e-1_f64) * t2629 * t9575;
    let t10586 = F::cast_from(0.32530743900905219526e-1_f64) * t2629 * t9572;
    (t10554, t10566, t10568, t10577, t10582, t10584, t10586)
}
