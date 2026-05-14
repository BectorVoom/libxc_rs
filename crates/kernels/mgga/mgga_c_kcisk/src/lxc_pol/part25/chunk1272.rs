//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1272/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1272<F: Float>(t11700: F, t2793: F, t33330: F, t111533: F, t2068: F, t32683: F, t22223: F, t2707: F, t5579: F, t9406: F, t32685: F, t34003: F, t34006: F, t34009: F, t34658: F, t34660: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t113203 = t2793 * t11700;
    let t113273 = 2.0 * t33330;
    let t113294 = 2.0 * t111533;
    let t113307 = t2068 * t32683;
    let t116002 = t22223 * t2707;
    let t116014 = 2.0 * t5579 * t9406;
    let t116034 = 4.0 * t32685;
    let t116053 = t34003 / 8.0;
    let t116054 = t34006 / 8.0;
    let t116055 = t34009 / 8.0;
    let t116056 = t34658 / 8.0;
    let t116057 = t34660 / 8.0;
    (t113203, t113273, t113294, t113307, t116002, t116014, t116034, t116053, t116054, t116055, t116056, t116057)
}
