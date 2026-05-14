//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1142/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1142<F: Float>(t2099: F, t2945: F, t7788: F, t7792: F, t7797: F, t2079: F, t297: F, t46: F, t18110: F, t18114: F, t18121: F, t18123: F, t2003: F, t2942: F, t2946: F, t2948: F, t5537: F, t655: F, t7350: F, t758: F, t7607: F, t7789: F, t7793: F, t7798: F) -> (F,) {
    let t21874 = t2945 * t2099 * t7788;
    let t21877 = t2945 * t2099 * t7792;
    let t21882 = t2945 * t2099 * t7797;
    let t21902 = t2079 * t297 * t46;
    let t21906 = 0.51448821741683684367e-2 * t21874 + 0.25724410870841842184e-2 * t21877 + 0.82318114786693894987e-1 * t7607 * t7798 - 0.10289764348336736874e-1 * t21882 - 0.3811023832717309953e-3 * t18110 + 0.28582678745379824648e-3 * t18114 + 0.12862205435420921092e-2 * t2945 * t758 * t2946 * t5537 + 0.38586616306262763276e-2 * t2945 * t758 * t2003 * t7350 * t655 - 0.41159057393346947493e-1 * t7607 * t7789 - 0.20579528696673473747e-1 * t7607 * t7793 - 77.0 / 486.0 * t18121 - 11.0 / 162.0 * t18123 + 0.13033701507893200039e0 * t2942 * t21902 * t2948;
    (t21906,)
}
