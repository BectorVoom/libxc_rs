//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 898/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk898<F: Float>(t1579: F, t21499: F, t1588: F, t6174: F, t3973: F, t6501: F, t1580: F, t6481: F, t6485: F, t3936: F, t4374: F, t13917: F, t6476: F, t2318: F, t4420: F, t4419: F, t6582: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t21500 = t1579 * t21499;
    let t21501 = t6174 * t1588;
    let t21510 = t3973 * t6501;
    let t21511 = t1580 * t21510;
    let t21513 = t3973 * t6481;
    let t21515 = 0.59969295720591057378e-2 * t1580 * t21513;
    let t21517 = t3973 * t6485;
    let t21519 = 0.11993859144118211476e-1 * t1580 * t21517;
    let t21524 = t3936 * t4374;
    let t21530 = t3936 * t1588;
    let t21536 = t13917 * t6476;
    let t21537 = t1580 * t21536;
    let t21555 = 0.17990788716177317213e-1 * t2318 * t4420;
    let t21556 = t4419 * t6582;
    (t21500, t21501, t21511, t21515, t21519, t21524, t21530, t21537, t21555, t21556)
}
