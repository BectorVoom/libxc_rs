//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 933/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk933<F: Float>(t1312: F, t839: F, t1310: F, t333: F, t335: F, t337: F, t339: F, t341: F, t343: F, t6709: F, t6711: F, t841: F, t843: F, t845: F, t847: F, t849: F) -> (F, F) {
    let t6715 = t1312 * t839;
    let t6745 = -0.64e0 * t6709 - 0.26112e1 * t6711 - 0.8704e0 * t333 * t6709 - 0.9214113627294e1 * t6715 - 0.27642340881882e2 * t841 * t1310 - 0.4607056813647e1 * t335 * t6709 + 0.734774460522e2 * t333 * t6715 + 0.1102161690783e3 * t843 * t1310 + 0.122462410087e2 * t337 * t6709 - 0.11494261417236e3 * t335 * t6715 - 0.11494261417236e3 * t845 * t1310 - 0.957855118103e1 * t339 * t6709 + 0.6202613620464e2 * t337 * t6715 + 0.4651960215348e2 * t847 * t1310 + 0.3101306810232e1 * t341 * t6709 - 0.1088826475632e2 * t339 * t6715 - 0.6532958853792e1 * t849 * t1310 - 0.362942158544e0 * t343 * t6709;
    (t6715, t6745)
}
