//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1117/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1117<F: Float>(t12748: F, t12752: F, t12815: F, t12817: F, t12819: F, t12930: F, t12965: F, t41138: F, t41139: F, t41140: F, t41141: F, t41142: F, t41143: F, t41144: F, t41145: F, t44058: F, t44104: F, t44144: F, t44536: F, t44879: F, t44922: F, t45109: F, t45110: F, t8: F) -> (F,) {
    let t45115 = -t41138 - t41139 + t8 * (t44058 + t44104 + t44144 + t44536 + t44879 + t44922 + t45109 + t45110) + t12748 - t41140 - t41141 - t12752 + t12815 - t12817 + t12819 + t12930 + t12965 + t41142 + t41143 + t41144 - t41145;
    (t45115,)
}
