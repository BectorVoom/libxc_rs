//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1342/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1342<F: Float>(t2349: F, t43: F, t10227: F, t96: F, t10199: F, t2175: F, t2289: F, t8264: F, t31377: F, t571: F, t1464: F, t8372: F, t31027: F, t31271: F, t116929: F, t8358: F) -> (F, F, F, F, F, F, F, F) {
    let t116942 = t43 * t2349;
    let t116946 = t96 * t10227;
    let t116968 = 154.0 / 27.0 * t10199 * t2175;
    let t116969 = t2289 * t8264;
    let t117369 = 2.0 * t571 * t31377;
    let t117374 = 2.0 * t8372 * t1464;
    let t117450 = 4.0 / 3.0 * t31027 * t31271;
    let t117457 = t116929 * t8358;
    (t116942, t116946, t116968, t116969, t117369, t117374, t117450, t117457)
}
