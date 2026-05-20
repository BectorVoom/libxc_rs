//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2143/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2143<F: Float>(t25260: F, t4368: F, t820: F, t844: F, t14914: F, t25270: F, t14919: F, t14904: F, t27261: F, t14900: F, t4462: F, t92951: F) -> (F, F, F, F, F, F) {
    let t98937 = t820 * t25260 * t844 * t4368;
    let t98940 = t25270 * t14914;
    let t98943 = t25270 * t14919;
    let t98945 = t27261 * t14904;
    let t98947 = t27261 * t14900;
    let t98949 = t92951 * t4462;
    (t98937, t98940, t98943, t98945, t98947, t98949)
}
