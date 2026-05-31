//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2240/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2240<F: Float>(t109118: F, t2014: F, t7237: F, t28167: F, t35669: F, t5627: F, t29996: F, t7235: F, t22483: F, t7312: F, t109078: F, t109081: F, t109087: F, t109090: F, t109092: F, t109095: F, t109099: F, t109103: F, t109107: F, t109110: F, t109112: F, t109117: F, t1843: F, t1911: F, t28160: F, t28230: F, t5517: F, t7725: F) -> F {
    let t109121 = F::cast_from(3.0_f64) * t2014 * t7237 * t109118;
    let t109124 = F::cast_from(12.0_f64) * t28167 * t35669 * t5627;
    let t109126 = F::cast_from(2.0_f64) * t7235 * t29996;
    let t109128 = t2014 * t7312 * t22483;
    let t109129 = -F::cast_from(2.0_f64) * t1843 * t28160 + F::cast_from(2.0_f64) * t1911 * t28230 - F::cast_from(2.0_f64) * t5517 * t7725 + t109078 - t109081 + t109087 + t109090 - t109092 - t109095 - t109099 + t109103 - t109107 + t109110 + t109112 - t109117 + t109121 + t109124 - t109126 - t109128;
    t109129
}
