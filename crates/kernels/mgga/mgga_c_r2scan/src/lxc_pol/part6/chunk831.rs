//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 831/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk831<F: Float>(t124: F, t717: F, t722: F, t1762: F, t1732: F, t1771: F, t230: F, t4889: F, t5836: F, t61: F, t1376: F, t725: F, t41: F, t1784: F, t584: F, t591: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5916 = t124 * t717;
    let t5917 = t5916 * t722;
    let t5919 = 0.64212977516902094772e0 * t1762 * t5917;
    let t5920 = t1771 * t1732;
    let t5923 = 120.0 * t4889 * t230;
    let t5925 = 0.3903689268108626343e0 * t61 * t5836;
    let t5926 = t1376 * t725;
    let t5927 = t41 * t5926;
    let t5930 = t584 * t1784 * t591;
    (t5916, t5917, t5919, t5920, t5923, t5925, t5926, t5927, t5930)
}
