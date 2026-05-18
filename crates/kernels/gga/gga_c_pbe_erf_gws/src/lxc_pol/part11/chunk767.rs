//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 767/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk767<F: Float>(t1031: F, t3491: F, t184: F, t221: F, t3390: F, t7027: F, t1621: F, t1620: F, t2612: F, t3500: F, t12339: F, t5008: F) -> (F, F, F, F, F, F, F, F) {
    let t12527 = t3491 * t1031;
    let t12528 = t12527 * t184;
    let t12530 = F::new(4.0) / F::new(5.0) * t12528 * t221;
    let t12531 = t7027 * t3390;
    let t12532 = t1621 * t12531;
    let t12534 = F::new(8.0) / F::new(5.0) * t1620 * t12532;
    let t12536 = F::new(8.0) / F::new(15.0) * t2612 * t3500;
    let t12537 = t5008 * t12339;
    (t12527, t12528, t12530, t12531, t12532, t12534, t12536, t12537)
}
