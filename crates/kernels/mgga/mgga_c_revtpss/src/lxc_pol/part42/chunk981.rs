//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 981/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk981<F: Float>(t10541: F, t786: F, t760: F, t9323: F, t9318: F, t2609: F, t717: F, t162: F, t9544: F, t158: F, t755: F, t9586: F, t2619: F, t2622: F, t2629: F, t9863: F) -> (F, F, F, F, F, F, F, F) {
    let t10542 = t786 * t10541;
    let t10552 = 0.51947577317044391277e2 * t760 * t9323;
    let t10554 = 0.35089341735807877242e1 * t760 * t9318;
    let t10563 = t717 * t2609;
    let t10565 = t162 * t9544;
    let t10566 = t158 * t10565;
    let t10568 = 0.56968947174242584612e-3 * t755 * t9586;
    let t10569 = t2622 * t2619;
    let t10577 = 0.16265371950452609763e-1 * t2629 * t9863;
    (t10542, t10552, t10554, t10563, t10566, t10568, t10569, t10577)
}
