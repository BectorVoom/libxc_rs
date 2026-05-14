//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1002/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1002<F: Float>(t2012: F, t7809: F, t9801: F, t2679: F, t7696: F, t9800: F, t2624: F, t7383: F, t1391: F, t825: F, t9850: F, t5840: F, t9890: F, t2017: F, t3295: F, t2033: F, t549: F, t9943: F) -> (F, F, F, F, F, F, F, F) {
    let t28673 = t2012 * t7809;
    let t28675 = 0.38342925953920749676e1 * t28673 * t9801;
    let t28678 = 0.38342925953920749676e1 * t9800 * t7696 * t2679;
    let t28681 = 0.19171462976960374838e1 * t9800 * t2624 * t7383;
    let t28683 = t825 * t1391 * t9850;
    let t28684 = 0.5396411800922179584e0 * t28683;
    let t28714 = t5840 * t9890;
    let t28715 = 0.1022478025437886658e1 * t28714;
    let t28726 = 0.11928910296775344344e1 * t825 * t2017 * t3295;
    let t28729 = 0.11916829983950142223e0 * t2033 * t549 * t9943;
    (t28673, t28675, t28678, t28681, t28684, t28715, t28726, t28729)
}
