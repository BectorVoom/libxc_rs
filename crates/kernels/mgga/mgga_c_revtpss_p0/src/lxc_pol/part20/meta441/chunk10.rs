//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1685/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1685<F: Float>(t33: F, t265: F, t502: F, t41211: F, t44088: F, t45901: F, t45903: F, t45908: F, t10326: F, t11095: F, t1113: F, t1304: F, t13196: F, t2258: F, t2838: F, t3351: F, t3805: F, t39457: F, t43744: F, t504: F, t57: F, t606: F, t895: F, t9357: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> F {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t45911 = piecewise3::<F>(t503, t44088 + t45901 + t45903 + t45908, t41211);
    let t45923 = piecewise3::<F>(t400, t41211 * t33 / F::new(2.0) + F::new(2.0) * t11095 * t1113 + F::new(3.0) * t2838 * t3351 + F::new(2.0) * t895 * t9357 + t265 * t43744 / F::new(2.0), t45911 * t57 / F::new(2.0) - F::new(2.0) * t13196 * t606 - F::new(3.0) * t3805 * t2258 - F::new(2.0) * t1304 * t10326 - t504 * t39457 / F::new(2.0));
    t45923
}
