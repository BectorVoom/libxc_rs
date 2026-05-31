//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 602/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk602<F: Float>(t30: F, t265: F, t502: F, t1966: F, t1996: F, t45: F, t1963: F, t33: F, t1940: F, t1995: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t503 = t265 < t502;
    let t1999 = piecewise3::<F>(t120, t1966, t1996 * t45 / F::cast_from(2.0_f64));
    let t2000 = t1963 * t33;
    let t2002 = t1940 * t2000 / F::cast_from(2.0_f64);
    let t2003 = piecewise3::<F>(t503, F::cast_from(0.0_f64), t1995);
    (t1999, t2002, t2003)
}
