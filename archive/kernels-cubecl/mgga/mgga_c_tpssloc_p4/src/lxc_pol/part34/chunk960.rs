//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 960/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk960<F: Float>(t22132: F, t974: F, t11759: F, t20234: F, t21745: F, t3440: F, t11649: F, t1174: F, t1726: F, t18310: F, t18312: F, t18314: F, t18321: F, t18325: F, t18327: F, t18330: F, t18333: F, t22012: F, t22015: F, t22116: F, t22119: F, t22129: F, t488: F, t4889: F, t6178: F, t6184: F, t6188: F) -> F {
    let t22133 = t974 * t22132;
    let t22136 = t11759 * t20234;
    let t22137 = t974 * t22136;
    let t22149 = t3440 * t21745;
    let t22152 = -F::cast_from(7.0_f64) / F::cast_from(648.0_f64) * t1174 * t22012 - t22015 * t488 / F::cast_from(192.0_f64) + t22116 * t488 / F::cast_from(3072.0_f64) - t1174 * t22119 / F::cast_from(48.0_f64) + t11649 - t4889 * t6178 / F::cast_from(27.0_f64) + t4889 * t6184 / F::cast_from(36.0_f64) + t4889 * t6188 / F::cast_from(18.0_f64) - t1174 * t22129 / F::cast_from(288.0_f64) - t1174 * t22133 / F::cast_from(48.0_f64) + t1174 * t22137 / F::cast_from(36.0_f64) + t18310 / F::cast_from(1536.0_f64) - t18312 / F::cast_from(144.0_f64) + F::cast_from(19.0_f64) / F::cast_from(864.0_f64) * t18314 - t18325 / F::cast_from(144.0_f64) + t18327 / F::cast_from(54.0_f64) - t18330 / F::cast_from(288.0_f64) + t18333 / F::cast_from(216.0_f64) - F::cast_from(11.0_f64) / F::cast_from(108.0_f64) * t18321 * t1726 + t1174 * t22149 / F::cast_from(72.0_f64);
    t22152
}
