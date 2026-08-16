//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 913/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk913<F: Float>(t242: F, t2675: F, t2725: F, t2722: F, t2732: F, t2731: F, t2458: F, t45: F, t2004: F, t924: F, t2685: F, t2689: F) -> (F, F, F, F, F) {
    let t8434 = t242 * t2675 * t2725;
    let t8435 = t2722 * t8434;
    let t8438 = t242 * t2675 * t2732;
    let t8439 = t2731 * t8438;
    let t8443 = t2458 * t45;
    let t8444 = F::cast_from(1.0_f64) / t8443;
    let t8450 = t2004 * t924;
    let t8453 = t2685 * t2689;
    (t8435, t8439, t8444, t8450, t8453)
}
