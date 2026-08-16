//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1297/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1297(t1704: f64, t9985: f64, t26686: f64, t2809: f64, t27837: f64, t2822: f64, t27856: f64, t7696: f64, t26685: f64, t27780: f64, t7703: f64, t8038: f64, t92818: f64, t92820: f64, t93562: f64, t93658: f64, t95645: f64, t95850: f64, t95852: f64, t95855: f64, t95860: f64) -> (f64, f64, f64) {
    let t95863 = t9985 * t1704;
    let t95865 = t26686 * t95863 * t2809;
    let t95868 = t2822 * t27837;
    let t95877 = 0.12356481481481481482e-2_f64 * t7696 * t27856;
    let t95878 = 0.66327777777777777776e-2_f64 * t95850 + 0.41188271604938271606e-3_f64 * t95852 + 0.51485339506172839507e-4_f64 * t95855 - 0.22653549382716049383e-2_f64 * t93658 * t8038 + 0.13901041666666666667e-2_f64 * t7703 * t95860 + 0.2782641015625e-3_f64 * t26685 * t95865 + 0.22109259259259259258e-2_f64 * t95868 - 0.18550940104166666667e-3_f64 * t26685 * t95645 + 0.11054629629629629629e-2_f64 * t92818 + 0.18424382716049382715e-2_f64 * t92820 + 0.4946917361111111111e-3_f64 * t93562 * t27780 - t95877;
    (t95865, t95868, t95878)
}
