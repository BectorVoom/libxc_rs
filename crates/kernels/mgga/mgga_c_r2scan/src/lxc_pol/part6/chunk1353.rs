//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1353/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1353<F: Float>(t2139: F, t2578: F, t6848: F, t20720: F, t7460: F, t6149: F, t7466: F, t2294: F, t8006: F, t2167: F, t259: F, t6217: F, t2568: F, t22744: F, t8098: F, t1551: F, t1570: F, t20511: F, t20792: F, t2122: F, t2124: F, t25191: F, t2557: F, t2562: F, t2598: F, t360: F, t495: F, t6132: F, t6219: F, t7380: F, t7433: F, t7503: F, t7514: F, t7944: F, t8001: F) -> (F, F) {
    let t25604 = t2139 * t6848 * t2578;
    let t25605 = 0.38140175656238781678e1 * t25604;
    let t25606 = t20720 * t7460;
    let t25611 = t6149 * t7466;
    let t25616 = t2139 * t2294 * t8006;
    let t25618 = t2167 * t259;
    let t25619 = t6217 * t25618;
    let t25632 = t2139 * t6848 * t2568;
    let t25633 = 0.38140175656238781678e1 * t25632;
    let t25634 = t22744 * t8098;
    let t25636 = 0.16463622957338778996e0 * t2122 * t2124 * t7503 * t7944 - 0.82318114786693894983e-1 * t2557 * t2124 * t25191 * t495 - 0.82318114786693894983e-1 * t2557 * t2124 * t7503 * t1551 + t25605 + 0.23404198698146525121e1 * t25606 * t360 * t2562 * t6219 - 0.69345773920434148506e0 * t25611 - 0.15602799132097683414e1 * t20511 * t7380 - 0.10401866088065122276e1 * t25616 - 0.15602799132097683414e1 * t25619 * t7514 + 0.78013995660488417067e0 * t2598 * t360 * t7433 * t1570 - 0.15423020329051080917e-3 * t20792 - 0.26004665220162805689e0 * t6132 * t360 * t8001 * t1570 + t25633 + 0.11524536070137145298e1 * t25634;
    (t25618, t25636)
}
