//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1053/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1053<F: Float>(t3827: F, t588: F, t3029: F, t3042: F, t3047: F, t3052: F, t3057: F, t3062: F, t584: F, t9954: F, t9959: F, t9962: F, t9967: F, t9970: F, t9975: F, t9978: F, t9983: F, t9986: F, t9991: F) -> (F, F) {
    let t9994 = t588 * t3827;
    let t9997 = t9954 * t584 / F::new(1152.0) - t3042 * t3029 / F::new(5760.0) - t9959 * t584 / F::new(11520.0) - t9962 * t584 / F::new(21504.0) + t3047 * t3029 / F::new(129024.0) + t9967 * t584 / F::new(258048.0) + t9970 * t584 / F::new(491520.0) - t3052 * t3029 / F::new(3440640.0) - t9975 * t584 / F::new(6881280.0) - t9978 * t584 / F::cast_from(13271040.0_f64) + t3057 * t3029 / F::new(0.10616832e9) + t9983 * t584 / F::new(0.21233664e9) + t9986 * t584 / F::cast_from(412876800.0_f64) - t3062 * t3029 / F::new(0.37158912e10) - t9991 * t584 / F::new(0.74317824e10) - F::new(2.0) / F::new(3.0) * t9994 * t584;
    (t9994, t9997)
}
