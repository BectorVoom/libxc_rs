//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1106/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1106<F: Float>(t5: F, t1493: F, t33275: F, t8621: F, t32798: F, t32802: F, t33283: F, t34402: F, t34410: F, t34761: F, t34765: F, t8737: F, t8882: F) -> (F, F) {
    let t7 = piecewise3::<F>(F::cast_from(0.0_f64) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t34771 = t8621 * t33275 * t1493;
    let t34775 = piecewise3::<F>(t8, F::cast_from(0.0_f64), -F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t34402 * t8882 + F::cast_from(5.0_f64) / F::cast_from(12.0_f64) * t32798 * t34761 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t32802 * t34765 - F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t34410 * t8882 - F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t8737 * t34771 + t33283);
    (t34771, t34775)
}
