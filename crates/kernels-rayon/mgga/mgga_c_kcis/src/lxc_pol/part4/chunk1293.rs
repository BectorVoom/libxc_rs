//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1293/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1293(t11369: f64, t16577: f64, t2642: f64, t518: f64, t5457: f64, t1419: f64, t5503: f64, t3255: f64, t5490: f64, t531: f64, t5526: f64, t833: f64) -> (f64, f64, f64, f64) {
    let t16579 = t11369 * t16577 * t2642;
    let t16582 = t5457 * t518;
    let t16584 = t16582 * t5503 * t1419;
    let t16587 = t3255 * t5490;
    let t16589 = t5526 * t531;
    let t16590 = t16589 * t833;
    (t16579, t16584, t16587, t16590)
}
