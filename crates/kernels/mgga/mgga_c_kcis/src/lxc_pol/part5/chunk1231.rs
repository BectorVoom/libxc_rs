//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1231/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1231<F: Float>(t3530: F, t6837: F, t1262: F, t5329: F, t3500: F, t6770: F, t1251: F, t15518: F, t15547: F, t15549: F, t20601: F, t20604: F, t20607: F, t20610: F, t20614: F, t3490: F, t3514: F, t6759: F, t6771: F) -> F {
    let t20617 = t3530 * t6837;
    let t20618 = t20617 * t1262;
    let t20619 = t5329 * t20618;
    let t20624 = t3500 * t6770;
    let t20625 = t1251 * t20624;
    let t20630 = -t3514 * t20601 / F::cast_from(432.0_f64) - t3514 * t20604 / F::cast_from(72.0_f64) + F::cast_from(7.0_f64) / F::cast_from(1296.0_f64) * t3514 * t20607 + t3514 * t20610 / F::cast_from(108.0_f64) - t3514 * t20614 / F::cast_from(288.0_f64) - t15518 + t1251 * t20619 / F::cast_from(96.0_f64) - t3490 * t6771 / F::cast_from(216.0_f64) + t20625 / F::cast_from(1728.0_f64) - t3490 * t6759 / F::cast_from(162.0_f64) - t15547 - t15549 / F::cast_from(1296.0_f64);
    t20630
}
