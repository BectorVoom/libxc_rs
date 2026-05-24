//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 879/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk879<F: Float>(t1417: F, t3561: F, t12924: F, t1422: F, t1423: F, t3559: F, t425: F, t1364: F, t5926: F, t1175: F, t459: F, t3587: F) -> (F, F, F, F, F) {
    let t13194 = t1417 * t3561;
    let t13197 = t1422 * t1423 * t12924;
    let t13200 = t425 * t3559;
    let t13201 = t13200 * t1364;
    let t13202 = t5926 * t13201;
    let t13205 = t459 * t1175;
    let t13206 = t13205 * t3587;
    (t13194, t13197, t13201, t13202, t13206)
}
