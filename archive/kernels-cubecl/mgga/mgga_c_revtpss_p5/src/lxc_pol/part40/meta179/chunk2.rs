//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 779/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk779<F: Float>(t3937: F, t3938: F, t3936: F, t159: F, t550: F, t216: F, t124: F, t3829: F, t800: F, t1376: F, t2689: F, t1353: F, t1413: F) -> (F, F, F, F, F, F) {
    let t3939 = t3937 * t3938;
    let t3940 = t3936 * t3939;
    let t3943 = t159 * t550;
    let t3944 = t216 * t3943;
    let t3945 = t124 * t3829;
    let t3946 = t800 * t3945;
    let t3950 = F::cast_from(0.76220476654346199061e-4_f64) * t2689 * t1376;
    let t3951 = t1413 * t1353;
    (t3940, t3943, t3944, t3946, t3950, t3951)
}
