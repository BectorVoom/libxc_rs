//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1265/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1265<F: Float>(t10141: F, t1819: F, t555: F, t10145: F, t10107: F, t1804: F, t6214: F, t3819: F, t6160: F, t3029: F, t584: F, t1196: F, t1198: F, t1200: F, t1202: F, t1204: F, t1206: F, t1208: F, t1880: F, t3855: F, t3857: F, t3859: F, t3861: F, t3863: F, t3865: F, t3867: F, t3869: F, t3871: F, t3873: F, t8036: F) -> (F, F, F, F, F) {
    let t27099 = t555 * t1819 * t10141;
    let t27102 = t555 * t1819 * t10145;
    let t27105 = t1804 * t6214 * t10107;
    let t27120 = t555 * t6160 * t3819;
    let t27139 = t584 * t3029;
    let t27176 = -t1200 * t27139 / F::new(20.0) + t1202 * t27139 / F::new(288.0) - t1204 * t27139 / F::new(5376.0) + t1206 * t27139 / F::new(122880.0) - t1208 * t27139 / F::new(3317760.0) + t8036 * t27139 / F::cast_from(103219200.0_f64) - F::new(8.0) / F::new(3.0) * t1196 * t27139 + t1198 * t27139 / F::new(2.0) + F::new(9.0) / F::new(80.0) * t3855 * t1880 - t3857 * t1880 / F::new(80.0) - F::new(11.0) / F::new(1152.0) * t3859 * t1880 + t3861 * t1880 / F::new(1152.0) + F::new(13.0) / F::new(21504.0) * t3863 * t1880 - t3865 * t1880 / F::new(21504.0) - t3867 * t1880 / F::new(32768.0) + t3869 * t1880 / F::new(491520.0) + F::new(17.0) / F::cast_from(13271040.0_f64) * t3871 * t1880 - t3873 * t1880 / F::cast_from(13271040.0_f64);
    (t27099, t27102, t27105, t27120, t27176)
}
