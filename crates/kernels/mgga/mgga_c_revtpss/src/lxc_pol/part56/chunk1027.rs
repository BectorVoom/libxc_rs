//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1027/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1027<F: Float>(t104527: F, t33517: F, t8937: F, t1294: F, t1469: F, t1811: F, t33468: F, t33494: F, t131395: F, t26894: F, t1042: F, t105270: F, t105460: F, t1238: F, t124619: F, t124644: F, t124646: F, t124675: F, t124814: F, t124827: F, t124928: F, t124950: F, t1252: F, t31993: F, t33425: F, t33498: F, t34918: F, t34920: F, t34961: F, t3626: F, t3719: F, t5347: F, t5405: F) -> (F, F) {
    let t131483 = t8937 * t104527 * t33517;
    let t131497 = t1469 * t1294;
    let t131503 = t33468 * t1811 * t33494;
    let t131506 = t26894 * t131395;
    let t131512 = -0.5578099381357651623e-3 * t124950 * t34920 + 0.16734298144072954869e-2 * t124814 * t31993 * t3719 * t105270 - 0.3718732920905101082e-3 * t131483 * t1252 - 0.17135921299530705785e1 * t124928 * t34961 - 0.24791552806034007214e-3 * t124619 - 0.11156198762715303246e-2 * t124675 * t1042 * t34918 * t5405 + 0.16734298144072954869e-2 * t124814 * t31993 * t3719 * t105460 + 0.18822977838986977999e-3 * t33425 * t3626 * t124827 * t131497 + 0.5578099381357651623e-3 * t131503 * t1238 + 0.29749863367240808656e-2 * t131506 * t33498 - 0.56468933516960933998e-3 * t124644 * t124646 * t5347;
    (t131497, t131512)
}
