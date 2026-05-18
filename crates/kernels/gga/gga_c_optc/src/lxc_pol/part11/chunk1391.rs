//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1391/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1391<F: Float>(t1102: F, t17790: F, t4224: F, t5219: F, t5307: F, t1512: F, t5239: F, t17454: F, t4305: F, t15562: F, t5268: F, t17502: F) -> (F, F, F, F, F, F) {
    let t58797 = F::new(0.46785787179641632568e1) * t1102 * t4224 * t17790;
    let t58800 = F::new(0.21053604230838734656e2) * t1102 * t5307 * t5219;
    let t58801 = t5239 * t1512;
    let t58812 = F::new(0.1403573615389248977e2) * t4305 * t17454;
    let t58820 = F::new(0.35089340384731224426e1) * t15562 * t5268;
    let t58822 = F::new(0.23392893589820816284e1) * t4305 * t17502;
    (t58797, t58800, t58801, t58812, t58820, t58822)
}
