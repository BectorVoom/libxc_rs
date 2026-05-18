//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 572/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk572<F: Float>(t3049: F, t313: F, t2963: F, t531: F, t2925: F, t808: F, t568: F, t836: F, t1040: F, t1044: F, t1049: F, t1998: F, t2004: F, t2009: F, t2049: F, t2103: F, t2194: F, t2197: F, t2639: F, t2721: F, t2725: F, t3015: F, t3019: F, t3022: F, t3025: F, t3028: F, t3032: F, t3035: F, t3040: F, t3043: F, t3046: F, t317: F, t780: F, t797: F, t807: F, t813: F, t833: F) -> (F, F, F, F, F, F, F) {
    let t3050 = t313 * t3049;
    let t3055 = t531 * t2963;
    let t3060 = t808 * t2925;
    let t3061 = t568 * t3060;
    let t3066 = t836 * t2925;
    let t3067 = t568 * t3066;
    let t3072 = F::new(0.71500979903700853338e0) * t2103 * t3015 - F::new(0.46011511144704899612e1) * t813 * t3019 + F::new(0.11502877786176224903e2) * t833 * t3022 - F::new(0.10725146985555128001e1) * t3025 * t2639 + F::new(0.23005755572352449806e1) * t807 * t3028 - F::new(0.23005755572352449806e1) * t1998 * t3032 - F::new(0.35750489951850426669e0) * t3035 * t2009 + F::new(0.35750489951850426669e0) * t780 * t3040 + F::new(0.35750489951850426669e0) * t2004 * t3043 + F::new(0.35750489951850426669e0) * t3046 * t317 + F::new(0.35750489951850426669e0) * t3050 * t317 - F::new(0.35750489951850426669e0) * t2049 * t1040 - F::new(0.35750489951850426669e0) * t797 * t3055 - F::new(0.23005755572352449806e1) * t2194 * t1044 - F::new(0.23005755572352449806e1) * t813 * t3061 + F::new(0.23005755572352449806e1) * t2197 * t1049 + F::new(0.23005755572352449806e1) * t833 * t3067 - F::new(0.19171462976960374838e0) * t2721 + F::new(0.42603251059911944084e-1) * t2725;
    (t3050, t3055, t3060, t3061, t3066, t3067, t3072)
}
