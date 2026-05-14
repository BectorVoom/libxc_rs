//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1454/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1454<F: Float>(t1024: F, t1026: F, t1028: F, t1030: F, t10496: F, t10502: F, t10504: F, t10508: F, t10510: F, t10514: F, t10516: F, t10520: F, t2410: F, t2418: F, t2422: F, t2426: F, t2430: F, t2956: F, t2962: F, t2966: F, t2970: F, t2974: F, t839: F, t9707: F) -> (F,) {
    let t35205 = -0.9214113627294e1 * t10496 * t839 + 0.2204323381566e3 * t2962 * t2410 + 0.1102161690783e3 * t2418 * t2956 + 0.1102161690783e3 * t1024 * t9707 + 0.367387230261e2 * t10502 * t839 - 0.22988522834472e3 * t10504 * t839 - 0.34482784251708e3 * t2966 * t2410 - 0.11494261417236e3 * t2422 * t2956 - 0.11494261417236e3 * t1026 * t9707 - 0.3831420472412e2 * t10508 * t839 + 0.18607840861392e3 * t10510 * t839 + 0.18607840861392e3 * t2970 * t2410 + 0.4651960215348e2 * t2426 * t2956 + 0.4651960215348e2 * t1028 * t9707 + 0.1550653405116e2 * t10514 * t839 - 0.4355305902528e2 * t10516 * t839 - 0.3266479426896e2 * t2974 * t2410 - 0.6532958853792e1 * t2430 * t2956 - 0.6532958853792e1 * t1030 * t9707 - 0.2177652951264e1 * t10520 * t839;
    (t35205,)
}
