//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1018/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1018<F: Float>(t10782: F, t701: F, t1445: F, t3504: F, t5782: F, t8483: F, t935: F, t2087: F, t2530: F, t3009: F, t10721: F, t10006: F, t10993: F, t10995: F, t10996: F, t11001: F, t11006: F, t11010: F, t11011: F, t11012: F, t11015: F, t11018: F, t1998: F, t2009: F, t780: F, t807: F) -> (F, F, F, F, F, F, F, F) {
    let t11019 = t10782 * t701;
    let t11020 = t1445 * t11019;
    let t11024 = F::cast_from(0.69017266717057349418e1_f64) * t5782 * t3504;
    let t11025 = t8483 * t935;
    let t11026 = t1445 * t11025;
    let t11028 = F::cast_from(0.69017266717057349418e1_f64) * t2087 * t11026;
    let t11029 = t3009 * t2530;
    let t11030 = t1445 * t11029;
    let t11032 = F::cast_from(0.69017266717057349418e1_f64) * t2087 * t11030;
    let t11033 = t1445 * t10721;
    let t11036 = -t10993 + t10995 - F::cast_from(0.35750489951850426669e0_f64) * t10996 * t2009 + F::cast_from(0.35750489951850426669e0_f64) * t780 * t11001 - F::cast_from(0.69017266717057349418e1_f64) * t2087 * t11006 + t11010 + t10006 - t11011 + t11012 + t11015 - t11018 - F::cast_from(0.23005755572352449806e1_f64) * t1998 * t11020 - t11024 - t11028 - t11032 + F::cast_from(0.23005755572352449806e1_f64) * t807 * t11033;
    (t11019, t11020, t11025, t11026, t11029, t11030, t11033, t11036)
}
