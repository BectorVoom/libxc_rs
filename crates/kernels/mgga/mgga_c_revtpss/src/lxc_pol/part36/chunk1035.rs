//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1035/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1035<F: Float>(t3920: F, t7246: F, t2023: F, t2453: F, t3908: F, t1426: F, t786: F, t25953: F, t7284: F, t25304: F, t7283: F, t25946: F, t3999: F, t2282: F, t55: F, t10309: F, t7565: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t26040 = 0.13009920719177044025e-1 * t7246 * t3920;
    let t26041 = t2453 * t2023;
    let t26043 = 0.11565819519348392139e-2 * t26041 * t3908;
    let t26053 = t2023 * t1426;
    let t26054 = t786 * t26053;
    let t26058 = 0.96373646535613327357e-2 * t7284 * t25953;
    let t26069 = t25304 * t7283;
    let t26071 = 0.22849835011101738147e-2 * t26069 * t25946;
    let t26079 = t1426 * t3999;
    let t26776 = t55 * t2282;
    let t26792 = t10309 * t7565;
    (t26040, t26041, t26043, t26053, t26054, t26058, t26069, t26071, t26079, t26776, t26792)
}
