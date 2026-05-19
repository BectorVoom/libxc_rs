//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1105/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1105<F: Float>(t2679: F, t7696: F, t9800: F, t2624: F, t7383: F, t1391: F, t825: F, t9850: F, t5840: F, t9890: F, t2017: F, t3295: F) -> (F, F, F, F, F) {
    let t28678 = F::cast_from(0.38342925953920749676e1_f64) * t9800 * t7696 * t2679;
    let t28681 = F::cast_from(0.19171462976960374838e1_f64) * t9800 * t2624 * t7383;
    let t28683 = t825 * t1391 * t9850;
    let t28714 = t5840 * t9890;
    let t28726 = F::cast_from(0.11928910296775344344e1_f64) * t825 * t2017 * t3295;
    (t28678, t28681, t28683, t28714, t28726)
}
