//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1828/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1828<F: Float>(t4003: F, t91921: F, t46478: F, t1390: F, t4002: F, t47203: F, t48947: F, t49030: F, t74585: F, t828: F, t86156: F, t86165: F, t86169: F, t86183: F, t86203: F, t86208: F, t86212: F, t86220: F, t86222: F, t86226: F, t86234: F, t86236: F) -> (F, F, F) {
    let t92177 = t91921 * t4003;
    let t92182 = t91921 * t46478;
    let t92195 = F::cast_from(0.6046824481244798459e0_f64) * t48947 - F::cast_from(0.24009450146119052704e-1_f64) * t86156 + F::cast_from(0.34299214494455789577e-2_f64) * t86165 + F::cast_from(0.11433071498151929859e-3_f64) * t86169 - F::cast_from(0.17149607247227894789e-2_f64) * t86183 + F::new(455.0) / F::new(162.0) * t49030 - F::cast_from(0.34013387707001991332e-1_f64) * t74585 + F::cast_from(0.30011812682648815881e-2_f64) * t4002 * t1390 * t828 * t92177 + F::cast_from(0.51448821741683684368e-2_f64) * t47203 * t1390 * t828 * t92182 + F::cast_from(0.28582678745379824648e-4_f64) * t86203 + F::cast_from(0.17149607247227894789e-3_f64) * t86208 - F::cast_from(0.17149607247227894789e-3_f64) * t86212 - F::cast_from(0.2032800112371413129e-3_f64) * t86220 + F::cast_from(0.48018900292238105409e0_f64) * t86222 - F::cast_from(0.6098400337114239387e-2_f64) * t86226 + F::cast_from(0.85748036236139473944e-4_f64) * t86234 + F::new(7.0) / F::new(3.0) * t86236;
    (t92177, t92182, t92195)
}
