//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1354/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1354<F: Float>(t535: F, t7907: F, t10080: F, t9722: F, t11901: F, t2922: F, t11780: F, t2849: F, t4083: F, t3740: F, t7926: F, t9746: F, t11889: F, t11659: F, t11843: F, t11846: F, t11849: F, t1520: F, t24218: F, t24661: F, t2839: F, t28701: F, t3741: F, t4526: F, t4529: F, t4554: F, t505: F, t511: F, t529: F, t7818: F, t7848: F, t7917: F, t9737: F, t9742: F, t9996: F, sigma2: F) -> (F, F, F, F, F, F) {
    let t33186 = t535 * t7907 * sigma2;
    let t33187 = t10080 * t9722;
    let t33190 = t11901 * t2922;
    let t33198 = t11780 * t4083 * t2849;
    let t33201 = t7926 * t3740;
    let t33205 = t10080 * t9746;
    let t33208 = t11889 * t2922;
    let t33232 = 0.5376e1 * t33186 * t33187 - 0.17208888888888888889e-2 * t3741 * t33190 + 0.1152e-4 * t7848 * t11780 * t4083 * t2839 + 0.16128e-4 * t7818 * t33198 - 0.192e1 * t33201 * t10080 * t9742 - 0.4032e1 * t28701 * t33205 - 0.3696e-2 * t7818 * t33208 - 0.192e-3 * t24218 * t4526 + 3200.0 / 27.0 * t11843 * t9737 - 1600.0 / 9.0 * t11846 * t9737 + 8000.0 / 9.0 * t11849 * t9737 + 3024.0 * t529 * t24661 * t4529 * t2849 + 24.0 * t505 * t11659 * t2849 - 24.0 * t511 * t7917 * t4554 * t2849 - 12.0 * t1520 * t9996;
    (t33187, t33190, t33198, t33205, t33208, t33232)
}
