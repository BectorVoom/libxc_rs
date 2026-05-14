//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1332/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1332<F: Float>(t1930: F, t2454: F, t11226: F, t33120: F, t5283: F, t735: F, t17861: F, t654: F, t5277: F, t2585: F, t1872: F, t6973: F, t1934: F, t10024: F, t5531: F, t2041: F, t34608: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t117349 = t1930 * t2454;
    let t117362 = t11226 * t33120;
    let t117369 = t5283 * t735;
    let t117385 = t17861 * t654;
    let t117400 = t5277 * t2454;
    let t117409 = sigma2 * t2585;
    let t117410 = t1872 * t117409;
    let t117419 = t6973 * t33120;
    let t117426 = t1934 * t735;
    let t117552 = t10024 * t5531;
    let t117560 = t34608 * t2041;
    (t117349, t117362, t117369, t117385, t117400, t117409, t117410, t117419, t117426, t117552, t117560)
}
