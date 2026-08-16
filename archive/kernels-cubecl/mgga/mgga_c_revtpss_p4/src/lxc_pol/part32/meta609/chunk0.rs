//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1948/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1948<F: Float>(t106625: F, t25207: F, t27375: F, t63185: F, t11064: F, t1544: F, t27384: F, t25759: F, t77425: F, t100987: F, t29598: F, t94245: F) -> (F, F, F, F, F, F, F) {
    let t106626 = t25207 * t106625;
    let t107793 = t63185 * t27375;
    let t107805 = t11064 * t1544 * t27384;
    let t107882 = t25759 * t77425;
    let t107885 = t100987 * t27375;
    let t107892 = t25759 * t106625;
    let t107895 = t94245 * t29598;
    (t106626, t107793, t107805, t107882, t107885, t107892, t107895)
}
