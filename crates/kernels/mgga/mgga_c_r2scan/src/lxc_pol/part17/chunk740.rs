//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 740/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk740<F: Float>(t1754: F, t2788: F, t2782: F, t584: F, t591: F, t1871: F, t956: F, t1859: F, t970: F, t5377: F, t2461: F, t60: F, t170: F, t1669: F, t2799: F, t585: F) -> (F, F, F, F, F, F, F) {
    let t7745 = t2788 * t1754;
    let t7751 = 0.1143056e0 * t584 * t2782 * t591;
    let t7753 = t584 * t956 * t1871;
    let t7755 = t1859 * t970;
    let t7756 = t7755 * t5377;
    let t7760 = t60 * t2461;
    let t7761 = t7760 * t170;
    let t7776 = t2799 * t1669;
    let t7778 = t2461 * t585;
    (t7745, t7751, t7753, t7756, t7761, t7776, t7778)
}
