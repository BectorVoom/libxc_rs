//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1133/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1133<F: Float>(t1762: F, t518: F, t717: F, t722: F, t5957: F, t5967: F, t1763: F, t5794: F, t1416: F, t1793: F, t190: F, t21062: F, t650: F, t390: F, t5381: F, t5767: F) -> (F, F, F, F, F, F) {
    let t21344 = 0.19977370783036207262e1 * t1762 * t518 * t717 * t722;
    let t21345 = t5967 * t5957;
    let t21349 = 0.43374325201206959368e-1 * t1762 * t1763 * t5794;
    let t21350 = t1416 * t1793;
    let t21354 = 18.0 * t650 * t190 * t21062;
    let t21357 = 0.34367190188705947437e1 * t390 * t5767 * t5381;
    (t21344, t21345, t21349, t21350, t21354, t21357)
}
