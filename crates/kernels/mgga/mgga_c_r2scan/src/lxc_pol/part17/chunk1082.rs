//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1082/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1082<F: Float>(t38150: F, t38170: F, t38177: F, t38657: F, t38661: F, t40238: F, t40248: F, t40251: F, t41762: F, t41763: F, t43677: F, t43682: F, t38183: F, t38666: F, t41775: F, t41776: F, t43688: F, t43690: F, t43692: F, t43695: F, t43697: F, t43700: F, t43702: F, t43705: F) -> (F, F) {
    let t44500 = 0.43663693315433241794e-2 * t43677 + 0.16262400898971305031e-3 * t38150 - t40238 - 0.86682217400542685632e-1 * t43682 - t41762 - t41763 + t38657 + t40248 + 0.45022119329691164871e0 * t38170 + t38661 - 0.65854491829355115986e-1 * t38177 - 0.7141495379651092646e0 * t40251;
    let t44510 = -0.10975748638225852664e0 * t43688 + 0.17336443480108537126e0 * t43690 + 0.5854464323629669811e-1 * t43692 - 0.32927245914677557993e-1 * t38183 + t38666 + t41775 - 0.25610080155860322883e0 * t43695 - 0.86682217400542685632e-1 * t43697 - 0.86682217400542685632e-1 * t43700 - 0.86682217400542685632e-1 * t43702 - t41776 + 0.13099107994629972538e-1 * t43705;
    (t44500, t44510)
}
