//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1037/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1037<F: Float>(t5: F, t10309: F, t8736: F, t136: F, t7565: F, t2247: F, t7574: F, t8435: F, t32151: F, t32586: F, t32593: F, t32602: F, t32795: F, t8623: F, t8737: F) -> (F, F, F, F, F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t32798 = t10309 * t8736;
    let t32801 = t7565 * t136;
    let t32802 = t2247 * t32801;
    let t32805 = t8435 * t7574;
    let t32806 = t2247 * t32805;
    let t32814 = piecewise3::<F>(t8, F::new(0.0), F::new(5.0) / F::new(144.0) * t32795 * t8623 - F::new(5.0) / F::new(24.0) * t32798 * t32586 - F::new(5.0) / F::new(36.0) * t32802 * t32593 + F::new(5.0) / F::new(144.0) * t32806 * t8623 + F::new(5.0) / F::new(72.0) * t8737 * t32602 + F::new(5.0) / F::new(144.0) * t8737 * t32151);
    (t32798, t32801, t32802, t32805, t32806, t32814)
}
