//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1765/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1765<F: Float>(t1131: F, t1150: F, t90529: F, t90542: F, t90558: F, t90573: F, t6439: F, t68792: F, t24262: F, t58342: F, t12227: F, t3435: F, t90324: F) -> (F, F, F, F) {
    let t90578 = F::new(1.0) * t1131 * (t90529 + t90542 + t90558 + t90573) * t1150;
    let t90580 = F::new(12.0) * t68792 * t6439;
    let t90582 = F::cast_from(0.3859675079686208416e3_f64) * t58342 * t24262;
    let t90585 = F::cast_from(0.57895126195293126241e3_f64) * t12227 * t90324 * t3435;
    (t90578, t90580, t90582, t90585)
}
