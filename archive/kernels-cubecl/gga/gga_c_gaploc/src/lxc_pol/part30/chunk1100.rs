//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1100/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1100<F: Float>(t2194: F, t9981: F, t2012: F, t7809: F, t9801: F, t2679: F, t7696: F, t9800: F, t2624: F, t7383: F, t1391: F, t825: F, t9850: F) -> (F, F, F, F, F, F) {
    let t28659 = t2194 * t9981;
    let t28673 = t2012 * t7809;
    let t28675 = F::cast_from(0.38342925953920749676e1_f64) * t28673 * t9801;
    let t28678 = F::cast_from(0.38342925953920749676e1_f64) * t9800 * t7696 * t2679;
    let t28681 = F::cast_from(0.19171462976960374838e1_f64) * t9800 * t2624 * t7383;
    let t28683 = t825 * t1391 * t9850;
    (t28659, t28673, t28675, t28678, t28681, t28683)
}
