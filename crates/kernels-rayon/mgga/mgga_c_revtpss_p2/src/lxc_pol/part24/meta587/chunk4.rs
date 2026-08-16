//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1828/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1828(t4003: f64, t91921: f64, t46478: f64, t1390: f64, t4002: f64, t47203: f64, t48947: f64, t49030: f64, t74585: f64, t828: f64, t86156: f64, t86165: f64, t86169: f64, t86183: f64, t86203: f64, t86208: f64, t86212: f64, t86220: f64, t86222: f64, t86226: f64, t86234: f64, t86236: f64) -> (f64, f64, f64) {
    let t92177 = t91921 * t4003;
    let t92182 = t91921 * t46478;
    let t92195 = 0.6046824481244798459e0_f64 * t48947 - 0.24009450146119052704e-1_f64 * t86156 + 0.34299214494455789577e-2_f64 * t86165 + 0.11433071498151929859e-3_f64 * t86169 - 0.17149607247227894789e-2_f64 * t86183 + 455.0_f64 / 162.0_f64 * t49030 - 0.34013387707001991332e-1_f64 * t74585 + 0.30011812682648815881e-2_f64 * t4002 * t1390 * t828 * t92177 + 0.51448821741683684368e-2_f64 * t47203 * t1390 * t828 * t92182 + 0.28582678745379824648e-4_f64 * t86203 + 0.17149607247227894789e-3_f64 * t86208 - 0.17149607247227894789e-3_f64 * t86212 - 0.2032800112371413129e-3_f64 * t86220 + 0.48018900292238105409e0_f64 * t86222 - 0.6098400337114239387e-2_f64 * t86226 + 0.85748036236139473944e-4_f64 * t86234 + 7.0_f64 / 3.0_f64 * t86236;
    (t92177, t92182, t92195)
}
