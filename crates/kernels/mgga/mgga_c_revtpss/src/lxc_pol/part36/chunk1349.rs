//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1349/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1349<F: Float>(t30: F, t265: F, t393: F, t1916: F, t30191: F, t30194: F, t114401: F, t117: F, t572: F, t114089: F, t113492: F, t1469: F, t2129: F, t22671: F, t30727: F, t45: F, t5825: F, t8161: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t114877 = F::new(18.0) * t1916 * t30191;
    let t114879 = F::new(9.0) * t1916 * t30194;
    let t114882 = F::new(3.0) * t572 * t117 * t114401;
    let t116053 = piecewise3::<f64>(t394, F::new(0.0), t114089);
    let t116063 = piecewise3::<f64>(t120, t113492, t116053 * t45 / F::new(2.0) + F::new(3.0) / F::new(2.0) * t30727 * t1469 + F::new(3.0) / F::new(2.0) * t8161 * t5825 + t2129 * t22671 / F::new(2.0));
    (t114877, t114879, t114882, t116063)
}
