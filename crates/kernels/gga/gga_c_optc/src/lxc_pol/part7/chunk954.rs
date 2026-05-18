//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 954/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk954<F: Float>(t9103: F, t9104: F, t3126: F, t8974: F, t9097: F, t1162: F, t3103: F, t3234: F, t3244: F, t4435: F, t4457: F, t4464: F, t9059: F, t9063: F, t9066: F, t9070: F, t9075: F, t9078: F, t9081: F, t9085: F, t9088: F, t9093: F, t9094: F, t9099: F, t9102: F) -> F {
    let t9105 = t9103 * t9104;
    let t9108 = t8974 * t3126;
    let t9109 = t9097 * t9108;
    let t9112 = F::new(0.11360101276506094136e1) * t3244 * t9059 - F::new(0.15486228121497046737e2) * t3103 * t9063 + F::new(0.1169609647897054359e2) * t3234 * t9066 + F::new(0.1949349413161757265e2) * t3234 * t9070 + F::new(0.4645868436449114021e2) * t4435 * t9075 + F::new(0.90151304338550081454e-1) * t1162 * t9078 - F::new(0.23181763972770020946e0) * t1162 * t9081 + F::new(0.28977204965962526182e-1) * t9085 + F::new(0.38636273287950034909e-1) * t9088 + t9093 + F::new(0.28977204965962526182e-1) * t1162 * t9094 - F::new(0.13186481011862155443e4) * t4464 * t9099 + F::new(0.56690705297447127569e5) * t9102 * t9105 + F::new(0.26372962023724310886e4) * t4457 * t9109;
    t9112
}
