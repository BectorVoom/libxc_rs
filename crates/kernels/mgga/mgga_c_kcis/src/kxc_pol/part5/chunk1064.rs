//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1064/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1064<F: Float>(t5036: F, t5189: F, t10491: F, t6638: F, t1203: F, t10498: F, t1820: F, t3330: F, t3325: F, t6735: F, t1808: F, t3436: F, t5182: F, t19630: F, t3438: F, t3437: F) -> (F, F, F, F, F, F, F, F) {
    let t19833 = 2.0 * t5036 * t5189;
    let t19835 = 2.0 * t10491 * t6638;
    let t19836 = t6638 * t1203;
    let t19838 = 6.0 * t10498 * t19836;
    let t19839 = t1820 * t5189;
    let t19841 = 4.0 * t3330 * t19839;
    let t19842 = t3325 * t6735;
    let t19843 = t6735 * t1203;
    let t19845 = 2.0 * t3330 * t19843;
    let t19846 = t1808 * t3436;
    let t19847 = t19846 * t5182;
    let t19849 = t3438 * t19630;
    let t19850 = t3437 * t19849;
    (t19833, t19835, t19838, t19841, t19842, t19845, t19847, t19850)
}
