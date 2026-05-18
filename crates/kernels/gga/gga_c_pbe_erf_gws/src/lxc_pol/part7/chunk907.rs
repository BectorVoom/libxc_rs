//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 907/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk907<F: Float>(t17105: F, t108: F, t1729: F, t267: F, t5214: F, t1733: F, t1816: F, t5211: F, t5212: F, t4897: F, t5213: F, t5145: F) -> (F, F, F, F, F) {
    let t17106 = F::new(128.0) / F::new(405.0) * t17105;
    let t17108 = t1729 * t108 * t267;
    let t17110 = F::new(64.0) / F::new(15.0) * t17108 * t5214;
    let t17114 = F::new(32.0) / F::new(15.0) * t5211 * t5212 * t1733 * t1816;
    let t17117 = F::new(32.0) / F::new(15.0) * t5211 * t5213 * t4897;
    let t17120 = F::new(32.0) / F::new(15.0) * t5211 * t5213 * t5145;
    (t17106, t17110, t17114, t17117, t17120)
}
