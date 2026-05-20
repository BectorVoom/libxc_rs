//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1193/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1193<F: Float>(t5: F, t30: F, t265: F, t393: F, t30714: F, t117: F, t2126: F, t5883: F, t29930: F, t1469: F, t2129: F, t29726: F, t45: F, t5825: F, t8161: F, t2142: F, t6587: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t30715 = piecewise3::<F>(t8, F::new(0.0), t30714);
    let t30716 = t30715 * t117;
    let t30724 = t2126 * t5883;
    let t30727 = piecewise3::<F>(t394, F::new(0.0), t29930);
    let t30734 = piecewise3::<F>(t120, t29726, t30727 * t45 / F::new(2.0) + t8161 * t1469 + t2129 * t5825 / F::new(2.0));
    let t30735 = t2142 * t6587;
    (t30715, t30716, t30724, t30727, t30734, t30735)
}
