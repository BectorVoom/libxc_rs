//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 433/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk433<F: Float>(t33: F, t265: F, t502: F, t1032: F, t1276: F, t2142: F, t473: F, t2144: F, t2149: F, t460: F, t1300: F, t198: F, t1995: F, t336: F, t2002: F, t57: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t2150 = t1032 * t1276;
    let t2151 = t473 * t2142;
    let t2152 = t2150 * t2151;
    let t2155 = F::new(0.65854491829355115987e0) * t460 * t2144 - F::new(0.4336814094102599731e0) * t2149 * t2152;
    let t2159 = piecewise3::<f64>(t503, t198 * t336 * t2155 * t1300, t1995);
    let t2162 = piecewise3::<f64>(t400, t2002, t2159 * t57 / F::new(2.0));
    (t2150, t2152, t2155, t2159, t2162)
}
