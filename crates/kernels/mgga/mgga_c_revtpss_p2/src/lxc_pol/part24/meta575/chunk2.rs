//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1761/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1761<F: Float>(t24324: F, t5063: F, t24327: F, t58473: F, t12230: F, t44017: F, t90324: F, t68255: F, t68257: F, t81156: F, t81158: F, t89839: F, t89851: F, t89865: F, t89869: F, t89873: F, t89877: F, t90379: F, t90384: F, t90387: F, t90390: F) -> (F, F, F, F) {
    let t90509 = F::new(4.0) * t5063 * t24324;
    let t90511 = F::cast_from(0.2069040516770936012e4_f64) * t58473 * t24327;
    let t90514 = F::cast_from(0.62071215503128080361e4_f64) * t44017 * t90324 * t12230;
    let t90529 = F::cast_from(0.43816888888888888889e0_f64) * t90379 + F::cast_from(0.79724444444444444446e0_f64) * t68255 - F::cast_from(0.5314962962962962963e0_f64) * t68257 + F::new(0.295764e1) * t90384 + F::cast_from(0.65725333333333333332e0_f64) * t90387 + F::cast_from(0.21908444444444444444e0_f64) * t90390 + F::cast_from(0.79724444444444444444e0_f64) * t81156 - F::cast_from(0.23917333333333333333e1_f64) * t81158 - F::cast_from(0.59793333333333333333e0_f64) * t89839 + F::new(0.17938e1) * t89851 + F::cast_from(0.39862222222222222223e1_f64) * t89865 - F::cast_from(0.71752000000000000002e1_f64) * t89869 + F::new(0.71752e1) * t89873 + F::cast_from(0.29896666666666666667e0_f64) * t89877;
    (t90509, t90511, t90514, t90529)
}
