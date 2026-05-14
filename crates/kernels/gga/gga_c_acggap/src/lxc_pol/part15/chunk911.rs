//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 911/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk911<F: Float>(t7447: F, t8823: F, t7440: F, t8826: F, t30817: F, t8948: F, t8793: F, t1313: F, t30598: F, t721: F, t1322: F, t7859: F, t31612: F, t31619: F, t31625: F, t31627: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t35848 = t7447 * t8823;
    let t35850 = t7440 * t8826;
    let t35874 = t30817 * t8948;
    let t35876 = t30817 * t8793;
    let t35882 = t30598 * t1313 * t721;
    let t35885 = t7859 * t1322 * t721;
    let t35890 = 0.17149607247227894789e-2 * t31612;
    let t35891 = 0.18868855373762491241e-1 * t31619;
    let t35893 = 0.25724410870841842184e-2 * t31625;
    let t35894 = 0.51448821741683684368e-2 * t31627;
    (t35848, t35850, t35874, t35876, t35882, t35885, t35890, t35891, t35893, t35894)
}
