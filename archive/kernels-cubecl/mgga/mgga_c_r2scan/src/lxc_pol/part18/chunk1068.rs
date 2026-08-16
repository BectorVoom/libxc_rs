//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1068/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1068<F: Float>(t120: F, t6511: F, t531: F, t10740: F, t776: F, t1050: F, t20621: F, t2090: F, t3294: F, t3296: F, t2096: F, t2167: F) -> (F, F, F, F, F, F, F) {
    let t37890 = t120 * t6511;
    let t37891 = t37890 * t531;
    let t37903 = t776 * t10740;
    let t37919 = t120 * t20621 * t1050;
    let t37920 = F::cast_from(0.92480845007273388189e0_f64) * t37919;
    let t37932 = t120 * t2090 * t3294;
    let t37933 = t37932 * t3296;
    let t37935 = t2167 * t2096;
    (t37890, t37891, t37903, t37920, t37932, t37933, t37935)
}
