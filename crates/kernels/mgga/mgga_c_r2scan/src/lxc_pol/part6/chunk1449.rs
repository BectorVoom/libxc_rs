//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1449/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1449<F: Float>(t498: F, t8296: F, t1563: F, t5086: F, t983: F, t11002: F, t1550: F, t1551: F, t1553: F, t1554: F, t1562: F, t19758: F, t2259: F, t2530: F, t2533: F, t2534: F, t2538: F, t2541: F, t2847: F, t495: F, t5065: F, t5074: F, t5087: F, t7197: F, t7202: F, t7204: F, t7206: F, t7218: F, t7221: F, t792: F, t920: F, t921: F) -> (F,) {
    let t27328 = t498 * t8296;
    let t27365 = t5086 * t983 * t1563;
    let t27371 = 3.0 * t2530 * t1553 * t2534 + 3.0 / 4.0 * t495 * t27328 + 3.0 / 4.0 * t1554 * t7221 + 3.0 / 2.0 * t7197 * t7206 + 3.0 / 4.0 * t7202 * t7206 - 15.0 / 16.0 * t2533 * t11002 * t1563 - 15.0 / 16.0 * t1562 * t8296 * t792 + 3.0 / 4.0 * t1551 * t7221 + t921 * t19758 / 4.0 + 3.0 / 4.0 * t7204 * t7206 + t5074 * t2538 / 4.0 - 15.0 / 16.0 * t1562 * t2847 * t2259 - 15.0 / 16.0 * t1551 * t7218 + 135.0 / 64.0 * t5087 * t2847 * t1563 + 3.0 * t2530 * t1550 * t2534 + t920 * t5065 * t2534 + 135.0 / 64.0 * t495 * t27365 + 135.0 / 64.0 * t5087 * t2541 * t2259;
    (t27371,)
}
