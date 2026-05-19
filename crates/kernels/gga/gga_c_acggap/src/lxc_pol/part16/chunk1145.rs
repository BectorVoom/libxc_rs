//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1145/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1145<F: Float>(t2288: F, t8901: F, t15386: F, t31195: F, t2001: F, t6076: F, t1998: F, t6081: F, t1856: F, t7614: F, t35259: F, t35261: F, t37464: F, t39797: F, t39802: F, t39807: F, t39809: F, t39811: F, t39813: F, t39815: F, t39817: F, t39819: F, t39822: F, t39825: F) -> (F, F) {
    let t39827 = t2288 * t8901;
    let t39829 = t31195 * t15386 * t39827;
    let t39831 = t2001 * t6076;
    let t39833 = t1998 * t6081;
    let t39835 = t7614 * t1856;
    let t39837 = t35259 + F::cast_from(0.64311027177104605458e-3_f64) * t39797 - t35261 - F::cast_from(0.31448092289604152068e-2_f64) * t39802 - F::cast_from(0.18868855373762491241e-1_f64) * t39807 - F::cast_from(0.17149607247227894789e-2_f64) * t39809 - F::cast_from(0.68598428988911579156e-2_f64) * t39811 + F::cast_from(0.34299214494455789578e-2_f64) * t39813 - F::cast_from(0.51448821741683684367e-2_f64) * t39815 + F::cast_from(0.17149607247227894789e-2_f64) * t39817 + F::cast_from(0.25724410870841842183e-2_f64) * t39819 + t39822 / F::new(24.0) + t39825 / F::new(192.0) + F::cast_from(0.47172138434406228102e-2_f64) * t39829 - F::cast_from(0.17149607247227894789e-2_f64) * t39831 + F::cast_from(0.85748036236139473944e-3_f64) * t39833 + F::cast_from(0.16006300097412701803e-1_f64) * t39835 - t37464;
    (t39827, t39837)
}
