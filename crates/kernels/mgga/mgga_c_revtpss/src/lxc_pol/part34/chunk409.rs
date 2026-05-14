//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 409/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk409<F: Float>(t30: F, t33: F, t265: F, t393: F, t502: F, t1978: F, t1983: F, t1986: F, t342: F, t1962: F, t207: F, t198: F, t892: F, t1102: F, t336: F, t1966: F, t45: F, t1963: F, t1940: F, t57: F, dens_threshold: F, rho0: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t1989 = 0.65854491829355115987e0 * t342 * t1978 - 0.4336814094102599731e0 * t1983 * t1986;
    let t1993 = t207 * t1962;
    let t1995 = t198 * t1993 * t892;
    let t1996 = piecewise3(t394, t198 * t336 * t1989 * t1102, t1995);
    let t1999 = piecewise3(t120, t1966, t1996 * t45 / 2.0);
    let t2000 = t1963 * t33;
    let t2002 = t1940 * t2000 / 2.0;
    let t2003 = piecewise3(t503, 0.0, t1995);
    let t2006 = piecewise3(t400, t2002, t2003 * t57 / 2.0);
    (t1989, t1993, t1996, t1999, t2000, t2003, t2006)
}
