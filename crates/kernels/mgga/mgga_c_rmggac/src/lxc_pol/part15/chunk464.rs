//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 464/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk464<F: Float>(t1173: F, t615: F, t495: F, t1525: F, t461: F, t1510: F, t4559: F, t217: F, t2184: F, t1465: F, t1470: F, t1494: F, t209: F) -> (F, F, F, F, F, F) {
    let t5554 = t1173 * t615;
    let t5555 = t5554 * t495;
    let t5558 = t461 * t1525;
    let t5571 = F::new(0.25610252642437845428e0) * t4559 * t1510;
    let t5582 = t2184 * t217;
    let t5583 = t1465 * t5582;
    let t5585 = F::new(0.25610252642437845428e0) * t5583 * t1470;
    let t5605 = t1494 * t209;
    (t5555, t5558, t5571, t5582, t5585, t5605)
}
