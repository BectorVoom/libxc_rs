//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 801/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk801<F: Float>(t10798: F, t5016: F, t5013: F, t1782: F, t7233: F, t4998: F, t5021: F, t1773: F, t1772: F, t4983: F, t4989: F, t4999: F, t1849: F, t569: F, t1310: F, t1764: F, t3934: F, t654: F) -> (F, F, F, F, F, F, F, F) {
    let t10799 = t10798 * t5016;
    let t10800 = t5013 * t10799;
    let t10802 = t7233 * t1782;
    let t10809 = t4998 * t5021;
    let t10810 = t1773 * t10809;
    let t10817 = t4983 * t1772;
    let t10828 = t4989 * t4999;
    let t10831 = 1.0 / t569 / t1849;
    let t10832 = t1310 * t10831;
    let t10856 = t1764 * t654 * t3934;
    (t10800, t10802, t10810, t10817, t10828, t10831, t10832, t10856)
}
