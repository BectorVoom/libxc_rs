//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1290/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1290(t12748: f64, t12752: f64, t12815: f64, t12817: f64, t12819: f64, t12930: f64, t12965: f64, t41138: f64, t41139: f64, t41140: f64, t41141: f64, t41142: f64, t41143: f64, t41144: f64, t41145: f64, t44058: f64, t44104: f64, t44144: f64, t44536: f64, t44879: f64, t44922: f64, t45109: f64, t45110: f64, t8: f64) -> f64 {
    let t45115 = -t41138 - t41139 + t8 * (t44058 + t44104 + t44144 + t44536 + t44879 + t44922 + t45109 + t45110) + t12748 - t41140 - t41141 - t12752 + t12815 - t12817 + t12819 + t12930 + t12965 + t41142 + t41143 + t41144 - t41145;
    t45115
}
