//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1364/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1364<F: Float>(t116393: F, t2063: F, t33225: F, t33226: F, t5520: F, t34580: F, t9736: F, t116423: F, t11966: F, t695: F, t5437: F, t18682: F, t964: F, t2028: F, t220: F, t112889: F, t116406: F, t117699: F, t33196: F, t33208: F, t33297: F, t34496: F, t34501: F, t9740: F) -> (F, F, F, F, F) {
    let t117913 = 0.25794135802469135802e-2 * t116393;
    let t117921 = t33225 * t33226 * t2063 * t5520;
    let t117925 = t34580 * t9736;
    let t117927 = 0.15476481481481481481e-2 * t116423;
    let t117928 = t11966 * t695;
    let t117931 = t33225 * t117928 * t2063 * t5437;
    let t117934 = t964 * t18682;
    let t117937 = t117934 * t33226 * t220 * t2028;
    let t117946 = t117913 + 0.13402777777777777778e-2 * t112889 + 0.34722222222222222222e-2 * t33297 * t34496 + 0.34722222222222222222e-2 * t33208 * t34496 + 0.17361111111111111111e-2 * t9740 * t117921 + 0.51588271604938271604e-3 * t116406 + 0.92592592592592592594e-2 * t117925 + t117927 - 0.20104166666666666667e-2 * t33196 * t117931 - 0.69444444444444444444e-2 * t9740 * t117937 + 0.69444444444444444444e-2 * t33297 * t34501 + 0.69444444444444444444e-2 * t33208 * t34501 - 0.10416666666666666667e-1 * t9740 * t117699;
    (t117921, t117931, t117934, t117937, t117946)
}
