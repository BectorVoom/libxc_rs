//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1060/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1060<F: Float>(t33: F, t265: F, t502: F, t33866: F, t1469: F, t33896: F, t57: F, t8553: F, t6985: F, t7742: F, t7935: F, t8568: F, t196: F, t197: F, t7894: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t33897 = piecewise3::<f64>(t503, F::new(0.0), t33866);
    let t33902 = piecewise3::<f64>(t400, t33896, -t8553 * t1469 / F::new(2.0) + t33897 * t57 / F::new(2.0));
    let t33906 = t6985 * t7742;
    let t33910 = t8568 * t7935;
    let t33913 = t7894 * t196 * t197;
    (t33897, t33902, t33906, t33910, t33913)
}
