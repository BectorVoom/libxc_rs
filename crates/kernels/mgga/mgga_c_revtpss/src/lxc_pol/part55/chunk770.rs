//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 770/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk770<F: Float>(t33: F, t265: F, t502: F, t2071: F, t7862: F, t8039: F, t1469: F, t1711: F, t1940: F, t2085: F, t2403: F, t57: F, t7432: F, t7869: F, t8020: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t8046 = t2071 * t7862;
    let t8059 = piecewise3::<f64>(t503, F::new(0.0), t8039);
    let t8064 = piecewise3::<f64>(t400, F::new(3.0) / F::new(2.0) * t2403 * t8046 + t1940 * t8020 * t33 / F::new(2.0) - t1940 * t7432 * t7869 / F::new(2.0) + t1940 * t2071 * t1711 / F::new(2.0), -t2085 * t1469 / F::new(2.0) + t8059 * t57 / F::new(2.0));
    (t8059, t8064)
}
