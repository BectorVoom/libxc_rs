//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1303/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1303<F: Float>(t10331: F, t160: F, t20396: F, t2042: F, t2098: F, t3997: F, t4014: F, t4017: F, t4019: F, t4021: F, t4023: F, t4025: F, t4027: F, t4029: F, t4031: F, t4033: F, t4035: F, t4037: F, t4039: F, t4041: F, t4043: F, t6270: F, t708: F) -> F {
    let t28376 = -t160 * t10331 * t708 / F::cast_from(5760.0_f64) - t4035 * t2042 / F::cast_from(21504.0_f64) - t4037 * t2042 / F::cast_from(32768.0_f64) + t4039 * t2042 / F::cast_from(491520.0_f64) + F::cast_from(17.0_f64) / F::cast_from(13271040.0_f64) * t4041 * t2042 - t4043 * t2042 / F::cast_from(13271040.0_f64) - F::cast_from(19.0_f64) / F::cast_from(412876800.0_f64) * t20396 * t3997 * t2042 + t6270 * t4014 * t2042 / F::cast_from(412876800.0_f64) + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t4017 * t2042 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t4019 * t2042 - F::cast_from(7.0_f64) / F::cast_from(8.0_f64) * t4021 * t2042 + t4023 * t2042 / F::cast_from(8.0_f64) + F::cast_from(9.0_f64) / F::cast_from(80.0_f64) * t4025 * t2042 - t4027 * t2042 / F::cast_from(80.0_f64) - F::cast_from(11.0_f64) / F::cast_from(1152.0_f64) * t4029 * t2042 + t4031 * t2042 / F::cast_from(1152.0_f64) + F::cast_from(13.0_f64) / F::cast_from(21504.0_f64) * t4033 * t2042 - t2098 * t10331 * t708 / F::cast_from(0.37158912e10_f64);
    t28376
}
