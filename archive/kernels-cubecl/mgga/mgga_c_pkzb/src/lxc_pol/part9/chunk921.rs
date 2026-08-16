//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 921/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk921<F: Float>(t2607: F, t501: F, t2605: F, t496: F, t5080: F, t5086: F, t5131: F, t5133: F, t5143: F, t5149: F, t126: F, t6798: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7012 = t501 * t2607;
    let t7013 = F::cast_from(8.0_f64) * t7012;
    let t7015 = F::cast_from(8.0_f64) * t496 * t2605;
    let t7017 = F::cast_from(8.0_f64) * t501 * t2605;
    let t7018 = F::cast_from(16.0_f64) * t5080;
    let t7019 = F::cast_from(32.0_f64) * t5086;
    let t7020 = F::cast_from(4.0_f64) * t5131;
    let t7021 = F::cast_from(4.0_f64) * t5133;
    let t7022 = F::cast_from(48.0_f64) * t5143;
    let t7023 = F::cast_from(0.23392894490538584828e1_f64) * t5149;
    let t7024 = t6798 * t126;
    (t7013, t7015, t7017, t7018, t7019, t7020, t7021, t7022, t7023, t7024)
}
