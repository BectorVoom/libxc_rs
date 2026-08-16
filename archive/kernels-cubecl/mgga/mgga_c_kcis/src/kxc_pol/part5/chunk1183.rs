//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1183/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1183<F: Float>(t14665: F, t1820: F, t14668: F, t5039: F, t5036: F, t5189: F, t10491: F, t6638: F, t1203: F, t10498: F, t3330: F, t3325: F, t6735: F) -> (F, F, F, F, F, F, F) {
    let t19829 = F::cast_from(2.0_f64) * t14665 * t1820;
    let t19831 = F::cast_from(4.0_f64) * t14668 * t5039;
    let t19833 = F::cast_from(2.0_f64) * t5036 * t5189;
    let t19835 = F::cast_from(2.0_f64) * t10491 * t6638;
    let t19836 = t6638 * t1203;
    let t19838 = F::cast_from(6.0_f64) * t10498 * t19836;
    let t19839 = t1820 * t5189;
    let t19841 = F::cast_from(4.0_f64) * t3330 * t19839;
    let t19842 = t3325 * t6735;
    (t19829, t19831, t19833, t19835, t19838, t19841, t19842)
}
