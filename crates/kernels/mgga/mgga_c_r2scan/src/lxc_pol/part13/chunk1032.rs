//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1032/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1032<F: Float>(t39835: F, t37920: F, t39814: F, t39816: F, t39818: F, t39821: F, t39824: F, t39826: F, t39828: F, t39831: F, t39832: F, t27067: F, t3333: F, t37754: F, t565: F, t1060: F, t920: F) -> (F, F, F, F) {
    let t39836 = 0.46574606203128791246e-1 * t39835;
    let t39837 = -0.10975748638225852664e0 * t39814 + 0.59512461497092438715e-1 * t39816 - 0.43663693315433241792e-2 * t39818 + 0.86682217400542685632e-1 * t39821 - t39824 - t39826 - t39828 + t39831 - 0.14282990759302185291e-1 * t39832 - t37920 + t39836;
    let t39838 = t27067 * t3333;
    let t39840 = t565 * t37754;
    let t39841 = t1060 * t920;
    (t39837, t39838, t39840, t39841)
}
