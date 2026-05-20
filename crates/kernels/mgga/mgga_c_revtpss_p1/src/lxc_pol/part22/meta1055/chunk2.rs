//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3732/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3732<F: Float>(t12772: F, t21160: F, t3625: F, t12784: F, t12910: F, t13312: F, t17459: F, t17461: F, t17644: F, t20747: F, t20795: F, t21040: F, t21228: F, t21298: F, t3626: F, t3628: F, t3720: F, t44459: F, t44466: F, t44609: F, t5340: F, t5351: F, t5405: F, t57147: F, t57584: F, t57586: F, t57590: F, t57602: F) -> F {
    let t70857 = t3625 * t12772 * t21160;
    let t70872 = -F::cast_from(0.25724410870841842184e-2_f64) * t44609 * t3720 * t20747 * t5405 - F::cast_from(0.28582678745379824648e-3_f64) * t3625 * t3626 * t21040 * t17644 - F::cast_from(0.28582678745379824648e-3_f64) * t5340 * t3626 * t20795 * t44466 - F::cast_from(0.57165357490759649296e-3_f64) * t5340 * t3626 * t20795 * t44459 - F::cast_from(0.91464571985215438872e-2_f64) * t57147 * t17461 - F::cast_from(0.57165357490759649296e-3_f64) * t12784 * t21228 - F::cast_from(0.3811023832717309953e-3_f64) * t70857 + F::cast_from(0.31758531939310916275e-3_f64) * t57584 - F::cast_from(0.28582678745379824648e-3_f64) * t3625 * t3626 * t5351 * t3628 * t13312 + F::cast_from(0.85748036236139473944e-3_f64) * t12910 * t3720 * t21298 * t17459 - F::cast_from(0.57165357490759649296e-3_f64) * t57586 - F::cast_from(0.28582678745379824648e-3_f64) * t57590 - F::cast_from(0.15244095330869239812e-2_f64) * t57602;
    t70872
}
