//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 795/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk795<F: Float>(t1721: F, t424: F, t1707: F, t124: F, t717: F, t722: F, t1762: F, t1732: F, t1771: F, t230: F, t4889: F, t5836: F, t61: F, t1748: F, t1751: F, t1398: F, t745: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5910 = t424 * t1721;
    let t5912 = t424 * t1707;
    let t5916 = t124 * t717;
    let t5917 = t5916 * t722;
    let t5919 = 0.64212977516902094772e0 * t1762 * t5917;
    let t5920 = t1771 * t1732;
    let t5923 = 120.0 * t4889 * t230;
    let t5925 = 0.3903689268108626343e0 * t61 * t5836;
    let t5936 = t1751 * t1748;
    let t5938 = t1398 * t745;
    (t5910, t5912, t5916, t5917, t5919, t5920, t5923, t5925, t5936, t5938)
}
