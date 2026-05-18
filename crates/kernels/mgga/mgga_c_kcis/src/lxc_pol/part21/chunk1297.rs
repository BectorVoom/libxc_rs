//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1297/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1297<F: Float>(t1704: F, t9985: F, t26686: F, t2809: F, t27837: F, t2822: F, t27856: F, t7696: F, t26685: F, t27780: F, t7703: F, t8038: F, t92818: F, t92820: F, t93562: F, t93658: F, t95645: F, t95850: F, t95852: F, t95855: F, t95860: F) -> (F, F, F) {
    let t95863 = t9985 * t1704;
    let t95865 = t26686 * t95863 * t2809;
    let t95868 = t2822 * t27837;
    let t95877 = F::new(0.12356481481481481482e-2) * t7696 * t27856;
    let t95878 = F::new(0.66327777777777777776e-2) * t95850 + F::new(0.41188271604938271606e-3) * t95852 + F::new(0.51485339506172839507e-4) * t95855 - F::new(0.22653549382716049383e-2) * t93658 * t8038 + F::new(0.13901041666666666667e-2) * t7703 * t95860 + F::new(0.2782641015625e-3) * t26685 * t95865 + F::new(0.22109259259259259258e-2) * t95868 - F::new(0.18550940104166666667e-3) * t26685 * t95645 + F::new(0.11054629629629629629e-2) * t92818 + F::new(0.18424382716049382715e-2) * t92820 + F::new(0.4946917361111111111e-3) * t93562 * t27780 - t95877;
    (t95865, t95868, t95878)
}
