//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 416/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk416<F: Float>(t33: F, t265: F, t502: F, t1963: F, t1940: F, t1995: F, t57: F, t1999: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t2000 = t1963 * t33;
    let t2002 = t1940 * t2000 / F::new(2.0);
    let t2003 = piecewise3::<f64>(t503, F::new(0.0), t1995);
    let t2006 = piecewise3::<f64>(t400, t2002, t2003 * t57 / F::new(2.0));
    let t2007 = t1999 + t2006;
    (t2000, t2003, t2007)
}
