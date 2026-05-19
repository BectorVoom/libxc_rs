//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1227/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1227<F: Float>(t413: F, t20361: F, t20549: F, t1260: F, t286: F, t25: F, t6838: F, t1251: F, t10990: F, t15215: F, t15219: F, t15223: F, t15477: F, t15493: F, t15496: F, t20346: F, t20350: F, t3490: F, t3514: F, t6839: F) -> (F, F) {
    let t418 = F::new(0.0) < t413;
    let t20550 = t20361 + t20549;
    let t20552 = piecewise3::<F>(t418, t20550, -t20550);
    let t20553 = t1260 * t20552;
    let t20554 = t286 * t20553;
    let t20559 = t25 * t6838;
    let t20560 = t1251 * t20559;
    let t20562 = -t15215 - t15219 + t15223 + t3514 * t20346 / F::new(144.0) - t3514 * t20350 / F::new(216.0) + t10990 / F::new(864.0) + t15477 / F::new(432.0) - t1251 * t20554 / F::new(192.0) + t3490 * t6839 / F::new(72.0) - t20560 / F::new(576.0) + t15493 - t15496;
    (t20550, t20562)
}
