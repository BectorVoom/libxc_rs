//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1131/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1131<F: Float>(t1358: F, t9208: F, t1365: F, t20692: F, t6525: F, t1349: F, t9083: F, t2317: F, t6289: F, t1217: F, t3122: F, t1222: F) -> (F, F, F, F, F, F) {
    let t30145 = F::cast_from(0.12646669615856066488e-1_f64) * t1358 * t9208;
    let t30148 = F::cast_from(0.47425011059460249332e-2_f64) * t6525 * t1365 * t20692;
    let t30152 = F::cast_from(0.63233348079280332442e-2_f64) * t1349 * t9083;
    let t30169 = F::cast_from(0.47425011059460249332e-2_f64) * t6525 * t6289 * t2317;
    let t30171 = F::cast_from(0.73772239425827054516e-2_f64) * t1217 * t3122;
    let t30173 = F::cast_from(0.63233348079280332442e-2_f64) * t1222 * t3122;
    (t30145, t30148, t30152, t30169, t30171, t30173)
}
