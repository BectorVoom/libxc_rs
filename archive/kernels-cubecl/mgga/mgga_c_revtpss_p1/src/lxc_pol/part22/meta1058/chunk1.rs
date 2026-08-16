//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3754/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3754<F: Float>(t12787: F, t13392: F, t17244: F, t17351: F, t17396: F, t17602: F, t17672: F, t17729: F, t20770: F, t20921: F, t3604: F, t3625: F, t3720: F, t44551: F, t44902: F, t44906: F, t44917: F, t5373: F, t56981: F, t59094: F, t6421: F, t71460: F, t71470: F, t71476: F, t71480: F, t71490: F) -> F {
    let t71492 = F::cast_from(0.31758531939310916275e-4_f64) * t44902 + F::cast_from(0.6351706387862183255e-4_f64) * t44906 + t71460 / F::cast_from(81.0_f64) - F::cast_from(0.57165357490759649296e-3_f64) * t59094 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t5373 * t17244 + F::cast_from(0.6351706387862183255e-4_f64) * t44917 + F::cast_from(0.57165357490759649296e-3_f64) * t17351 * t56981 * t20770 + F::cast_from(0.3811023832717309953e-3_f64) * t71470 - F::cast_from(0.47637797908966374413e-3_f64) * t17729 * t12787 * t20921 * t13392 + F::cast_from(0.30488190661738479624e-2_f64) * t71476 + F::cast_from(0.22866142996303859718e-2_f64) * t17396 * t17602 + F::cast_from(0.85748036236139473944e-3_f64) * t44551 * t3720 * t71480 * t3604 + F::cast_from(0.23818898954483187207e-3_f64) * t3625 * t12787 * t6421 * t17672 - F::cast_from(0.17149607247227894789e-2_f64) * t71490;
    t71492
}
