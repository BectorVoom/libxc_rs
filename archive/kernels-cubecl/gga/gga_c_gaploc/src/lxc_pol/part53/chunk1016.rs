//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1016/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1016<F: Float>(t41649: F, t41654: F, t41657: F, t41661: F, t41667: F, t41670: F, t41674: F, t41677: F, t41681: F, t41683: F, t41686: F, t41690: F, t41692: F, t47829: F, t47832: F, t47835: F, t47839: F, t47846: F, t47850: F, t47854: F) -> F {
    let t50832 = F::cast_from(0.38342925953920749676e0_f64) * t47829 - F::cast_from(0.76685851907841499352e0_f64) * t47832 - F::cast_from(0.15889106645266856298e0_f64) * t47835 + F::cast_from(0.23833659967900284447e0_f64) * t47839 + F::cast_from(0.76685851907841499352e0_f64) * t41649 + t41654 - t41657 + t41661 - t41667 - t41670 - F::cast_from(0.76685851907841499352e0_f64) * t41674 - F::cast_from(0.50050685932590597338e1_f64) * t47846 - F::cast_from(0.11502877786176224903e1_f64) * t47850 - F::cast_from(0.79445533226334281487e-1_f64) * t47854 + t41677 + t41681 - F::cast_from(0.76685851907841499352e0_f64) * t41683 - F::cast_from(0.76685851907841499352e0_f64) * t41686 + t41690 - t41692;
    t50832
}
