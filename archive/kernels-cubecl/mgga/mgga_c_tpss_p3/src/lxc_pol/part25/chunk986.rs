//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 986/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk986<F: Float>(t30: F, t33: F, t13334: F, t13583: F, t13588: F, t1989: F, t4360: F, t490: F, t580: F, t5335: F, t9868: F, t3289: F, t5059: F, t1006: F, t4368: F, t493: F, zeta_threshold: F) -> (F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t13594 = piecewise3::<F>(t31, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t13583 * t580 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t4360 * t1989 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t13588 * t580 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t490 * t13334);
    let t13595 = t9868 * t5335;
    let t13600 = t3289 * t5059;
    let t13603 = -t13334;
    let t13607 = piecewise3::<F>(t34, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t13595 * t1006 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t4368 * t1989 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t13600 * t1006 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t493 * t13603);
    (t13594, t13603, t13607)
}
