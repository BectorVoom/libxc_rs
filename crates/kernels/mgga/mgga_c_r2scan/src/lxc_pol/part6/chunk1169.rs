//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1169/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1169<F: Float>(t1762: F, t5527: F, t5960: F, t595: F, t6021: F, t637: F, t518: F, t704: F, t706: F, t717: F, t722: F, t5957: F, t5967: F, t1763: F, t5794: F, t1416: F, t1793: F) -> (F, F, F, F, F, F, F) {
    let t21333 = 0.11558335953042377059e2 * t1762 * t5960 * t5527;
    let t21335 = t595 * t6021 * t637;
    let t21340 = 0.13494234507042165137e0 * t1762 * t518 * t704 * t706;
    let t21344 = 0.19977370783036207262e1 * t1762 * t518 * t717 * t722;
    let t21345 = t5967 * t5957;
    let t21349 = 0.43374325201206959368e-1 * t1762 * t1763 * t5794;
    let t21350 = t1416 * t1793;
    (t21333, t21335, t21340, t21344, t21345, t21349, t21350)
}
