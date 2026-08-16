//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1267/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1267<F: Float>(t1913: F, t3037: F, t3042: F, t3047: F, t3052: F, t3057: F, t3062: F, t7984: F, t9950: F, t9954: F, t9959: F, t9962: F, t9967: F, t9970: F, t9975: F, t9978: F, t9983: F, t9986: F, t9991: F, t9994: F) -> F {
    let t27260 = t3037 * t7984 / F::cast_from(320.0_f64) + t9950 * t1913 / F::cast_from(640.0_f64) + t9954 * t1913 / F::cast_from(1152.0_f64) - t3042 * t7984 / F::cast_from(5760.0_f64) - t9959 * t1913 / F::cast_from(11520.0_f64) - t9962 * t1913 / F::cast_from(21504.0_f64) + t3047 * t7984 / F::cast_from(129024.0_f64) + t9967 * t1913 / F::cast_from(258048.0_f64) + t9970 * t1913 / F::cast_from(491520.0_f64) - t3052 * t7984 / F::cast_from(3440640.0_f64) - t9975 * t1913 / F::cast_from(6881280.0_f64) - t9978 * t1913 / F::cast_from(13271040.0_f64) + t3057 * t7984 / F::cast_from(0.10616832e9_f64) + t9983 * t1913 / F::cast_from(0.21233664e9_f64) + t9986 * t1913 / F::cast_from(412876800.0_f64) - t3062 * t7984 / F::cast_from(0.37158912e10_f64) - t9991 * t1913 / F::cast_from(0.74317824e10_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t9994 * t1913;
    t27260
}
