//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 829/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk829<F: Float>(t1026: F, t1028: F, t1030: F, t2410: F, t2966: F, t2968: F, t2970: F, t2972: F, t2974: F, t2976: F, t343: F, t839: F, t9707: F, t9738: F, t9746: F, t2441: F, t352: F) -> (F, F) {
    let t9756 = -0.11494261417236e3 * t2966 * t839 - 0.7662840944824e2 * t1026 * t2410 - 0.3831420472412e2 * t2968 * t839 - 0.957855118103e1 * t9738 + 0.6202613620464e2 * t2970 * t839 + 0.3101306810232e2 * t1028 * t2410 + 0.1550653405116e2 * t2972 * t839 + 0.3101306810232e1 * t9746 - 0.1088826475632e2 * t2974 * t839 - 0.4355305902528e1 * t1030 * t2410 - 0.2177652951264e1 * t2976 * t839 - 0.362942158544e0 * t343 * t9707;
    let t9760 = t352 * t2441;
    (t9756, t9760)
}
