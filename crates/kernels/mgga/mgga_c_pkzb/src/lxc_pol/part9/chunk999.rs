//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 999/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk999<F: Float>(t7969: F, t8002: F, t852: F, t833: F, t2336: F, t3147: F, t1171: F, t2196: F, t2199: F, t2317: F, t3135: F, t3161: F) -> (F, F, F, F, F, F, F, F) {
    let t8003 = t7969 + t8002;
    let t8004 = t8003 * t852;
    let t8006 = F::cast_from(1.0_f64) * t833 * t8004;
    let t8008 = F::cast_from(0.5848223622634646207e0_f64) * t3147 * t2336;
    let t8009 = t1171 * t2196;
    let t8011 = F::cast_from(2.0_f64) * t8009 * t2199;
    let t8012 = t2317 * t3135;
    let t8013 = t8012 * t3161;
    (t8003, t8004, t8006, t8008, t8009, t8011, t8012, t8013)
}
