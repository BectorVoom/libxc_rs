//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 864/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk864<F: Float>(t10803: F, t2124: F, t5115: F, t3295: F, t10753: F, t10759: F, t10762: F, t10765: F, t10770: F, t10774: F, t10778: F, t10782: F, t10786: F, t10790: F, t10794: F, t10797: F, t10801: F) -> (F, F) {
    let t10804 = 0.10975748638225852664e-1 * t10803;
    let t10805 = t2124 * t5115;
    let t10806 = t3295 * t10805;
    let t10808 = 0.86682217400542685632e-1 * t10753 + t10759 - 0.43663693315433241792e-2 * t10762 - 0.13099107994629972538e-1 * t10765 + 0.47609969197673950972e-2 * t10770 + 0.2600466522016280569e0 * t10774 + 0.86682217400542685632e-1 * t10778 + 0.10975748638225852664e0 * t10782 - 0.43663693315433241792e-2 * t10786 - 0.26198215989259945075e-1 * t10790 + 0.21831846657716620896e-2 * t10794 + 0.13099107994629972538e-1 * t10797 + 0.65495539973149862688e-2 * t10801 + t10804 - 0.27439371595564631661e-1 * t10806;
    (t10805, t10808)
}
