//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1316/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1316<F: Float>(t11700: F, t2793: F, t33330: F, t2068: F, t32683: F, t5579: F, t9406: F, t34003: F, t34006: F, t34009: F, t34658: F, t34660: F, t34663: F, t34665: F, t34668: F, t34671: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t113203 = t2793 * t11700;
    let t113273 = 2.0 * t33330;
    let t113307 = t2068 * t32683;
    let t116014 = 2.0 * t5579 * t9406;
    let t116053 = t34003 / 8.0;
    let t116054 = t34006 / 8.0;
    let t116055 = t34009 / 8.0;
    let t116056 = t34658 / 8.0;
    let t116057 = t34660 / 8.0;
    let t116058 = t34663 / 8.0;
    let t116059 = t34665 / 8.0;
    let t116061 = t34668 / 8.0;
    let t116062 = t34671 / 8.0;
    (t113203, t113273, t113307, t116014, t116053, t116054, t116055, t116056, t116057, t116058, t116059, t116061, t116062)
}
