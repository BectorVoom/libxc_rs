//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 919/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk919<F: Float>(t1047: F, t1656: F, t25498: F, t25539: F, t27448: F, t27450: F, t27460: F, t27462: F, t27464: F, t27467: F, t27496: F, t27518: F, t27541: F, t375: F, t4803: F, t4808: F, t7132: F) -> F {
    let t27543 = F::new(0.28582678745379824648e-3) * t27448 + F::new(0.42874018118069736972e-3) * t27450 * t1047 - F::new(0.57165357490759649296e-3) * t7132 * t4803 + F::new(0.47637797908966374413e-3) * t7132 * t4808 - F::new(0.28582678745379824648e-3) * t25498 - t25539 * t1656 / F::new(108.0) + t27460 / F::new(864.0) + F::new(0.28582678745379824648e-3) * t27462 - F::new(0.22866142996303859718e-2) * t27464 * t375 + F::new(0.42874018118069736972e-3) * t27467 * t375 + t27496 + t27518 + t27541;
    t27543
}
