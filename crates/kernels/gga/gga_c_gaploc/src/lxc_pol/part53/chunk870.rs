//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 870/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk870<F: Float>(t42546: F, t1358: F, t3394: F, t488: F, t9065: F, t25718: F, t9194: F, t41993: F, t6507: F, t10272: F, t2317: F, t6525: F) -> (F, F, F, F, F) {
    let t42547 = F::cast_from(0.47425011059460249332e-2_f64) * t42546;
    let t42551 = F::cast_from(0.31616674039640166221e-2_f64) * t1358 * t9065 * t3394 * t488;
    let t42570 = F::cast_from(0.37940008847568199464e-1_f64) * t1358 * t25718 * t9194;
    let t42573 = F::cast_from(0.63233348079280332442e-2_f64) * t1358 * t6507 * t41993;
    let t42579 = t6525 * t10272 * t2317;
    (t42547, t42551, t42570, t42573, t42579)
}
