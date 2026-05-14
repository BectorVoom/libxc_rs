//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1342/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1342<F: Float>(t12561: F, t28: F, t5778: F, t89: F, t23608: F, t27160: F, t458: F, t27154: F, t95053: F, t2075: F, t446: F, t6630: F, t9432: F, t1369: F, t1637: F, t6669: F) -> (F, F, F, F, F, F, F) {
    let t105843 = t89 * t28 * t5778 * t12561;
    let t105846 = t23608 * t458 * t27160;
    let t105847 = t105846 / 4.0;
    let t105848 = t95053 * t27154;
    let t105849 = t105848 / 3.0;
    let t105853 = t446 * t9432 * t6630 * t2075;
    let t105856 = t1369 * t1637 * t6669;
    (t105843, t105846, t105847, t105848, t105849, t105853, t105856)
}
