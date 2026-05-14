//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1264/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1264<F: Float>(t5037: F, t963: F, t1048: F, t2262: F, t6967: F, t795: F, t2810: F, t6887: F, t2854: F, t2858: F, t6334: F, t19062: F, t19064: F, t19336: F, t19041: F, t19048: F, t19057: F, t19061: F, t19069: F) -> (F, F, F, F, F, F, F) {
    let t23768 = t963 * t5037;
    let t23769 = 0.10389515463408878255e3 * t23768;
    let t23773 = 6.0 * t1048 * t6967 * t795 * t2262;
    let t23774 = t6887 * t2810;
    let t23775 = 0.7089e1 * t23774;
    let t23778 = 18.0 * t2858 * t2854 * t6334;
    let t23779 = 0.15584273195113317383e3 * t19062;
    let t23780 = 0.10526802520742363173e2 * t19064;
    let t23781 = 192.0 * t19336;
    let t23782 = -t23769 + t19041 + t19048 - t23773 - t23775 + t19057 - t23778 - t19061 + t23779 - t23780 - t19069 - t23781;
    (t23769, t23773, t23778, t23779, t23780, t23781, t23782)
}
