//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1556/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1556<F: Float>(t30: F, t265: F, t393: F, t41211: F, t41477: F, t41574: F, t41943: F, t43720: F, t10326: F, t1106: F, t11095: F, t12201: F, t2257: F, t2258: F, t2838: F, t3340: F, t39456: F, t39457: F, t395: F, t45: F, t605: F, t606: F, t895: F, t9344: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> F {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t43723 = piecewise3::<F>(t394, t41477 + t41574 + t41943 + t43720, t41211);
    let t43735 = piecewise3::<F>(t120, t41211 * t30 / F::new(2.0) + F::new(2.0) * t11095 * t605 + F::new(3.0) * t2838 * t2257 + F::new(2.0) * t895 * t9344 + t265 * t39456 / F::new(2.0), t43723 * t45 / F::new(2.0) + F::new(2.0) * t12201 * t606 + F::new(3.0) * t3340 * t2258 + F::new(2.0) * t1106 * t10326 + t395 * t39457 / F::new(2.0));
    t43735
}
