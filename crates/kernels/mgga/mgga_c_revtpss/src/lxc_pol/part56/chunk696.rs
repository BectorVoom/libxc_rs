//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 696/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk696<F: Float>(t265: F, t502: F, t1259: F, t72: F, t3720: F, t8926: F, t8928: F, t8932: F, t8933: F, t8938: F, t8941: F, t8946: F, t2155: F, t1300: F, t198: F, t336: F, t3801: F, t8542: F) -> (F, F, F, F, F) {
    let t503 = t265 < t502;
    let t8947 = t1259 * t72;
    let t8948 = t8947 * t3720;
    let t8951 = 0.28234466758480466999e-3 * t8926 * t8928 - 0.8673628188205199462e0 * t8932 * t8933 + 0.57119737665102352616e0 * t8938 * t8941 - 0.1859366460452550541e-3 * t8946 * t8948;
    let t8955 = t2155 * t2155;
    let t8960 = piecewise3(t503, t1300 * t198 * t336 * t8951 - t198 * t336 * t3801 * t8955, t8542);
    (t8947, t8948, t8951, t8955, t8960)
}
