//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1053/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1053<F: Float>(t10492: F, t335: F, t10498: F, t333: F, t337: F, t339: F, t341: F, t1022: F, t1024: F, t1026: F, t1028: F, t1030: F, t10494: F, t10496: F, t2956: F, t343: F) -> (F, F, F, F, F, F, F, F) {
    let t10502 = t335 * t10492;
    let t10504 = t333 * t10498;
    let t10508 = t337 * t10492;
    let t10510 = t335 * t10498;
    let t10514 = t339 * t10492;
    let t10516 = t337 * t10498;
    let t10520 = t341 * t10492;
    let t10528 = -0.64e0 * t10492 - 0.26112e1 * t10494 - 0.8704e0 * t10496 - 0.9214113627294e1 * t10498 - 0.27642340881882e2 * t1022 * t2956 - 0.4607056813647e1 * t10502 + 0.734774460522e2 * t10504 + 0.1102161690783e3 * t1024 * t2956 + 0.122462410087e2 * t10508 - 0.11494261417236e3 * t10510 - 0.11494261417236e3 * t1026 * t2956 - 0.957855118103e1 * t10514 + 0.6202613620464e2 * t10516 + 0.4651960215348e2 * t1028 * t2956 + 0.3101306810232e1 * t10520 - 0.1088826475632e2 * t339 * t10498 - 0.6532958853792e1 * t1030 * t2956 - 0.362942158544e0 * t343 * t10492;
    (t10502, t10504, t10508, t10510, t10514, t10516, t10520, t10528)
}
