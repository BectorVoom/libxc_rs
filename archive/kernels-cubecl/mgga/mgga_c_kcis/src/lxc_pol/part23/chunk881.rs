//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 881/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk881<F: Float>(t11369: F, t16577: F, t2642: F, t518: F, t5457: F, t1419: F, t5503: F, t3255: F, t5490: F, t531: F, t5526: F, t833: F) -> (F, F, F, F) {
    let t16579 = t11369 * t16577 * t2642;
    let t16582 = t5457 * t518;
    let t16584 = t16582 * t5503 * t1419;
    let t16587 = t3255 * t5490;
    let t16589 = t5526 * t531;
    let t16590 = t16589 * t833;
    (t16579, t16584, t16587, t16590)
}
