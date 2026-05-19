//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1262/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1262<F: Float>(t7978: F, t99236: F, t209: F, t2095: F, t7967: F, t27607: F, t28781: F, t98344: F, t18171: F, t28700: F, t27583: F, t27566: F, t28713: F) -> (F, F, F, F, F, F, F, F) {
    let t99238 = F::cast_from(0.23168402777777777778e-3_f64) * t7978 * t99236;
    let t99247 = t2095 * t209;
    let t99248 = t7967 * t99247;
    let t99260 = F::cast_from(0.23168402777777777778e-3_f64) * t27607 * t28781;
    let t99282 = F::cast_from(0.30952962962962962962e-2_f64) * t98344;
    let t99291 = t18171 * t28700;
    let t99293 = F::cast_from(0.7722800925925925926e-4_f64) * t27583 * t99291;
    let t99301 = t28713 * t27566;
    (t99238, t99247, t99248, t99260, t99282, t99291, t99293, t99301)
}
