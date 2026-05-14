//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 801/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk801<F: Float>(t9680: F, t9683: F, t9686: F, t9690: F, t9693: F, t9698: F, t9704: F, t9707: F, t9711: F, t9714: F, t9717: F, t9719: F, t9724: F, t9731: F, t9734: F, t9742: F, t9745: F, t9748: F, t9751: F, t9754: F, t9758: F, t9761: F, t9764: F, t9766: F, t9768: F, t9771: F) -> (F, F) {
    let t10915 = -0.73909120450717768468e-5 * t9680 + 0.15176747947735985782e-6 * t9683 - 0.2698425785107458272e-6 * t9686 - 0.51491428373437201896e-5 * t9690 + 0.4637672555408563478e-4 * t9693 - 0.75091666377929252765e-6 * t9698 - 0.66398272271344937795e-7 * t9704 + 0.1180561280984512994e-6 * t9707 - 0.18757833100512778952e-8 * t9711 + 0.25294579912893309636e-8 * t9714 + 0.10120442708333333334e-4 * t9717 + 0.27801896084645508334e-2 * t9719 + 0.16882049790461501058e-6 * t9724;
    let t10932 = 0.34752370105806885418e-3 * t9731 - 0.38647271295071362317e-7 * t9734 + 0.43047021936487268522e-6 * t9742 + 0.17376185052903442709e-3 * t9745 - 0.13900948042322754167e-3 * t9748 - 0.13900948042322754167e-3 * t9751 + 0.41702844126968262501e-3 * t9754 + 0.10005428175813516294e-8 * t9758 + 0.15458908518028544927e-5 * t9761 - 0.51491428373437201896e-5 * t9764 - 0.34752370105806885418e-3 * t9766 + 0.28960308421505737848e-5 * t9768 - 0.45018799441230669486e-7 * t9771;
    (t10915, t10932)
}
