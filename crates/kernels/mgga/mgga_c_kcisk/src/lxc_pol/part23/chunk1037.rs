//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1037/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1037<F: Float>(t1056: F, t5650: F, t3619: F, t5646: F, t1354: F, t220: F, t1364: F, t443: F, t5655: F, t1346: F, t5659: F, t5704: F, t2192: F, t3283: F, t3831: F, t5703: F) -> (F, F, F, F, F, F, F, F) {
    let t20832 = t5650 * t1056;
    let t20835 = t5646 * t3619;
    let t20838 = t1354 * t220;
    let t20839 = t20838 * t1364;
    let t20843 = 0.93706135855523581992e-2 * t443 * t5655;
    let t20845 = 0.28111840756657074598e-1 * t1346 * t5659;
    let t20846 = t5704 * t1056;
    let t20849 = t2192 * t3283;
    let t20852 = t3831 * t5703;
    (t20832, t20835, t20839, t20843, t20845, t20846, t20849, t20852)
}
