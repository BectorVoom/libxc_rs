//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 367/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk367<F: Float>(t30: F, t33: F, t1312: F, t1502: F, t1518: F, t1468: F, t513: F, t1711: F, t516: F, t162: F, zeta_threshold: F) -> (F, F) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t1847 = F::new(2.0) * t1312 * t1518 + t1502;
    let t1851 = piecewise3::<F>(t31, F::new(0.0), F::new(4.0) / F::new(3.0) * t513 * t1468);
    let t1854 = piecewise3::<F>(t34, F::new(0.0), F::new(4.0) / F::new(3.0) * t516 * t1711);
    let t1856 = (t1851 + t1854) * t162;
    (t1847, t1856)
}
