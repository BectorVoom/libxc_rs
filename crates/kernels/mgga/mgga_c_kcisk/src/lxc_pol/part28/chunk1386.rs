//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1386/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1386<F: Float>(t121456: F, t9664: F, t116223: F, t1869: F, t34221: F, t2563: F, t34159: F, t7069: F, t1333: F, t35221: F, t35224: F, t112623: F, t112661: F, t112663: F, t116212: F, t117062: F, t117065: F, t117084: F, t117106: F, t117121: F, t23820: F, t2785: F, t33031: F, t35237: F, t4830: F) -> (F, F, F, F, F) {
    let t121910 = t9664 * t121456;
    let t121915 = t1869 * t116223 * t34221;
    let t121919 = t1869 * t34159 * t2563 * t7069;
    let t121921 = t1333 * t35221;
    let t121928 = t1333 * t35224;
    let t121930 = t117062 + t117065 - 0.89351851851851851855e-3 * t112623 - 0.13888888888888888889e-1 * t33031 * t116212 * t23820 - 0.69444444444444444447e-2 * t121910 - 0.58958024691358024689e-2 * t117084 + 0.89351851851851851853e-3 * t117106 - 0.33163888888888888888e-2 * t121915 + 0.99491666666666666664e-2 * t121919 + t117121 - 0.88437037037037037033e-2 * t121921 + 0.55555555555555555558e-1 * t4830 * t35237 * t2785 + 0.55273148148148148147e-3 * t112661 - 0.36848765432098765431e-3 * t112663 + 0.16581944444444444444e-2 * t121928;
    (t121915, t121919, t121921, t121928, t121930)
}
