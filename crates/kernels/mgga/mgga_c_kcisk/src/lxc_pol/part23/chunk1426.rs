//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1426/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1426<F: Float>(t109820: F, t114260: F, t114268: F, t114273: F, t114276: F, t114280: F, t114293: F, t114296: F, t114302: F, t114308: F, t114334: F, t114337: F, t114341: F, t20: F, t2316: F, t2734: F, t2740: F, t32358: F, t3913: F, t9850: F) -> (F,) {
    let t115631 = 0.77382407407407407406e-3 * t114260 - 0.3574074074074074074e-2 * t109820 - 0.11607361111111111111e-2 * t114268 + 0.27777777777777777778e-1 * t9850 * t32358 * t2740 - 0.23214722222222222222e-2 * t114273 + 0.61905925925925925925e-2 * t114276 + 0.34822083333333333332e-2 * t114280 + 0.11607361111111111111e-2 * t114293 + 0.19345601851851851852e-2 * t114296 - 0.25794135802469135802e-3 * t114302 - 0.19345601851851851852e-2 * t114308 - 0.23214722222222222222e-2 * t114334 - 0.61905925925925925926e-2 * t114337 - 0.12381185185185185185e-1 * t114341 - 0.50925925925925925926e-1 * t2734 * t2316 * t3913 * t20 * t2740;
    (t115631,)
}
