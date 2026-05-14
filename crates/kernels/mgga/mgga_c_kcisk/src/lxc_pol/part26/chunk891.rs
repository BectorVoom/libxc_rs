//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 891/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk891<F: Float>(t443: F, t5647: F, t1354: F, t220: F, t5655: F, t1346: F, t5659: F, t2110: F, t3929: F, t140: F, t3737: F, t5631: F, t5636: F, t13959: F, t5628: F, t5622: F) -> (F, F, F, F, F, F, F, F, F) {
    let t20827 = 0.93706135855523581992e-2 * t443 * t5647;
    let t20838 = t1354 * t220;
    let t20843 = 0.93706135855523581992e-2 * t443 * t5655;
    let t20845 = 0.28111840756657074598e-1 * t1346 * t5659;
    let t20886 = t2110 * t3929;
    let t20890 = t140 * t3737 * t5631;
    let t20891 = t20890 * t5636;
    let t20892 = 0.3684876543209876543e-2 * t20891;
    let t20893 = t13959 * t5628;
    let t20895 = t13959 * t5622;
    (t20827, t20838, t20843, t20845, t20886, t20891, t20892, t20893, t20895)
}
