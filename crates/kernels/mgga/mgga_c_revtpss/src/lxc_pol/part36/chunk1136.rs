//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1136/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1136<F: Float>(t3920: F, t7246: F, t2023: F, t2453: F, t3908: F, t1426: F, t786: F, t25953: F, t7284: F, t25304: F, t7283: F, t25946: F) -> (F, F, F, F, F, F, F, F) {
    let t26040 = F::new(0.13009920719177044025e-1) * t7246 * t3920;
    let t26041 = t2453 * t2023;
    let t26043 = F::new(0.11565819519348392139e-2) * t26041 * t3908;
    let t26053 = t2023 * t1426;
    let t26054 = t786 * t26053;
    let t26058 = F::new(0.96373646535613327357e-2) * t7284 * t25953;
    let t26069 = t25304 * t7283;
    let t26071 = F::new(0.22849835011101738147e-2) * t26069 * t25946;
    (t26040, t26041, t26043, t26053, t26054, t26058, t26069, t26071)
}
