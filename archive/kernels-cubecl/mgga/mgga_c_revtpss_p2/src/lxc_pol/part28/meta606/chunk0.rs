//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2098/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2098<F: Float>(t26072: F, t27888: F, t27873: F, t94886: F, t27845: F, t689: F, t25904: F, t25899: F, t94649: F, t97685: F, t25898: F, t7925: F, t94849: F) -> (F, F, F, F, F, F) {
    let t97943 = F::cast_from(0.14456046980341999104e-1_f64) * t26072 * t27888;
    let t97945 = F::cast_from(0.51405703062096148812e-1_f64) * t94886 * t27873;
    let t97947 = t27845 * t689;
    let t97949 = F::cast_from(0.14456046980341999104e-1_f64) * t25904 * t97947;
    let t97951 = F::cast_from(0.25702851531048074406e-1_f64) * t25899 * t97947;
    let t97953 = F::cast_from(0.51405703062096148812e-1_f64) * t94649 * t97685;
    let t97956 = t94849 * t25898 * t7925;
    (t97943, t97945, t97949, t97951, t97953, t97956)
}
