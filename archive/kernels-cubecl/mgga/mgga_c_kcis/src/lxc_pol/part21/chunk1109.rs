//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1109/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1109<F: Float>(t1262: F, t1267: F, t26996: F, t5329: F, t2845: F, t7789: F, t3507: F, t3500: F, t7790: F, t7788: F, t2829: F, t1252: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26997 = t1262 * t1267;
    let t26998 = t26996 * t26997;
    let t26999 = t5329 * t26998;
    let t27002 = t7789 * t2845;
    let t27003 = t3507 * t27002;
    let t27006 = t3500 * t7790;
    let t27007 = t7788 * t27006;
    let t27009 = t7789 * t2829;
    let t27010 = t1252 * t27009;
    (t26997, t26998, t26999, t27002, t27003, t27006, t27007, t27009, t27010)
}
