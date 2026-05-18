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
    let t28376 = -t160 * t10331 * t708 / F::new(5760.0) - t4035 * t2042 / F::new(21504.0) - t4037 * t2042 / F::new(32768.0) + t4039 * t2042 / F::new(491520.0) + F::new(17.0) / F::new(13271040.0) * t4041 * t2042 - t4043 * t2042 / F::new(13271040.0) - F::new(19.0) / F::new(412876800.0) * t20396 * t3997 * t2042 + t6270 * t4014 * t2042 / F::new(412876800.0) + F::new(10.0) / F::new(3.0) * t4017 * t2042 - F::new(2.0) / F::new(3.0) * t4019 * t2042 - F::new(7.0) / F::new(8.0) * t4021 * t2042 + t4023 * t2042 / F::new(8.0) + F::new(9.0) / F::new(80.0) * t4025 * t2042 - t4027 * t2042 / F::new(80.0) - F::new(11.0) / F::new(1152.0) * t4029 * t2042 + t4031 * t2042 / F::new(1152.0) + F::new(13.0) / F::new(21504.0) * t4033 * t2042 - t2098 * t10331 * t708 / F::new(0.37158912e10);
    t28376
}
