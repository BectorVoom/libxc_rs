//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1746/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1746<F: Float>(t162: F, t47038: F, t47053: F, t189: F, t512: F, t1340: F, t40165: F, t2626: F, t9551: F, t749: F, t9363: F, t268: F, t520: F) -> (F, F, F, F, F, F) {
    let t47055 = (t47038 + t47053) * t162;
    let t47057 = t512 * t47055 * t189;
    let t47059 = F::cast_from(0.12304822629859687989e5_f64) * t1340 * t40165;
    let t47060 = t9551 * t2626;
    let t47061 = F::cast_from(0.70178683471615754484e1_f64) * t47060;
    let t47063 = t512 * t9363 * t749;
    let t47064 = F::new(4.0) * t47063;
    let t47065 = t520 * t268;
    (t47055, t47057, t47059, t47061, t47064, t47065)
}
