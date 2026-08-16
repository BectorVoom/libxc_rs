//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1371/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1371<F: Float>(t1692: F, t1812: F, t18728: F, t18807: F, t20012: F, t20041: F, t20058: F, t20417: F, t20526: F, t21495: F, t21516: F, t21659: F, t2439: F, t33: F, t3552: F, t5671: F, t5849: F, t6214: F, t6354: F, t66281: F, t66317: F, t70847: F, t70890: F, t70915: F, t70923: F, t70942: F, t70957: F, t72173: F, t72187: F, t72279: F) -> F {
    let t72495 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2439 * t21659 * t5671 - F::cast_from(3.0_f64) * t18728 * t70847 + F::cast_from(3.0_f64) * t3552 * t1812 * t70923 - t1692 * t18807 * t21516 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) * t18728 * t70890 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t18728 * t70915 + F::cast_from(2.0_f64) * t20526 * t70942 + F::cast_from(3.0_f64) * t2439 * t5849 * t21495 - F::cast_from(3.0_f64) * t20417 * t70957 + t72187 + F::cast_from(6.0_f64) * t72279 * t20012 + t1692 * t72173 * t33 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) * t66317 * t20041 - t1692 * t66281 * t6214 + F::cast_from(3.0_f64) * t2439 * t6354 * t20058;
    t72495
}
