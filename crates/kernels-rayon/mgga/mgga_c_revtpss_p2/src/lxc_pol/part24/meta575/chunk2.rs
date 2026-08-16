//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1761/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1761(t24324: f64, t5063: f64, t24327: f64, t58473: f64, t12230: f64, t44017: f64, t90324: f64, t68255: f64, t68257: f64, t81156: f64, t81158: f64, t89839: f64, t89851: f64, t89865: f64, t89869: f64, t89873: f64, t89877: f64, t90379: f64, t90384: f64, t90387: f64, t90390: f64) -> (f64, f64, f64, f64) {
    let t90509 = 4.0_f64 * t5063 * t24324;
    let t90511 = 0.2069040516770936012e4_f64 * t58473 * t24327;
    let t90514 = 0.62071215503128080361e4_f64 * t44017 * t90324 * t12230;
    let t90529 = 0.43816888888888888889e0_f64 * t90379 + 0.79724444444444444446e0_f64 * t68255 - 0.5314962962962962963e0_f64 * t68257 + 0.295764e1_f64 * t90384 + 0.65725333333333333332e0_f64 * t90387 + 0.21908444444444444444e0_f64 * t90390 + 0.79724444444444444444e0_f64 * t81156 - 0.23917333333333333333e1_f64 * t81158 - 0.59793333333333333333e0_f64 * t89839 + 0.17938e1_f64 * t89851 + 0.39862222222222222223e1_f64 * t89865 - 0.71752000000000000002e1_f64 * t89869 + 0.71752e1_f64 * t89873 + 0.29896666666666666667e0_f64 * t89877;
    (t90509, t90511, t90514, t90529)
}
