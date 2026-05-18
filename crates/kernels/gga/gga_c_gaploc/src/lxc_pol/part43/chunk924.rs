//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 924/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk924<F: Float>(t2021: F, t43572: F, t5974: F, t10817: F, t9972: F, t1445: F, t3209: F, t813: F, t8528: F, t10915: F, t22242: F, t43598: F) -> (F, F, F, F) {
    let t43781 = F::new(0.25025342966295298669e1) * t2021 * t43572 * t5974;
    let t43783 = F::new(0.50050685932590597338e1) * t10817 * t9972;
    let t43787 = F::new(0.92023022289409799224e1) * t813 * t1445 * t8528 * t3209;
    let t43790 = F::new(0.21450293971110256001e1) * t22242 * t10915 * t43598;
    (t43781, t43783, t43787, t43790)
}
