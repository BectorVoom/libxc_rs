//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1071/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1071<F: Float>(t5: F, t2035: F, t34399: F, t7935: F, t8764: F, t13272: F, t8736: F, t8142: F, t8435: F, t2247: F, t32798: F, t32802: F, t33621: F, t34173: F, t34177: F, t34181: F, t8623: F, t8737: F) -> (F, F, F, F, F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t34400 = t34399 * t2035;
    let t34401 = t8764 * t7935;
    let t34402 = t13272 * t8736;
    let t34409 = t8435 * t8142;
    let t34410 = t2247 * t34409;
    let t34418 = piecewise3::<F>(t8, F::new(0.0), F::new(5.0) / F::new(144.0) * t34402 * t8623 - F::new(5.0) / F::new(24.0) * t32798 * t34173 - F::new(5.0) / F::new(36.0) * t32802 * t34177 + F::new(5.0) / F::new(144.0) * t34410 * t8623 + F::new(5.0) / F::new(72.0) * t8737 * t34181 + F::new(5.0) / F::new(144.0) * t8737 * t33621);
    (t34400, t34401, t34402, t34409, t34410, t34418)
}
