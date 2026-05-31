//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1184/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1184<F: Float>(t1203: F, t6735: F, t3330: F, t1808: F, t3436: F, t5182: F, t19630: F, t3438: F, t3437: F, t1196: F, t6709: F, t10787: F, t6693: F) -> (F, F, F, F, F) {
    let t19843 = t6735 * t1203;
    let t19845 = F::cast_from(2.0_f64) * t3330 * t19843;
    let t19846 = t1808 * t3436;
    let t19847 = t19846 * t5182;
    let t19849 = t3438 * t19630;
    let t19850 = t3437 * t19849;
    let t19852 = t6709 * t1196;
    let t19854 = t10787 * t6693;
    (t19845, t19847, t19850, t19852, t19854)
}
