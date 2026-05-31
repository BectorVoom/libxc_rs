//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2777/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2777<F: Float>(t14746: F, t2703: F, t14923: F, t14927: F, t10811: F, t14697: F, t40672: F, t828: F, t10786: F, t10861: F, t14494: F, t14586: F, t14676: F, t14772: F, t14785: F, t14791: F, t14872: F, t2394: F, t2724: F, t2745: F, t2747: F, t36833: F, t40782: F, t40784: F, t40789: F, t40792: F, t40801: F, t40804: F, t40810: F, t40816: F, t4362: F, t4364: F, t4366: F, t4450: F, t50511: F, t50560: F, t836: F, t837: F) -> F {
    let t50982 = t2703 * t14746;
    let t51000 = t14923 * t14927;
    let t51006 = t10811 * t14697;
    let t51014 = t40672 * t828;
    let t51025 = F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t50982 - F::cast_from(0.51448821741683684367e-2_f64) * t4362 * t2747 * t4450 * t10861 + F::cast_from(0.45351183609335988443e0_f64) * t40782 + F::cast_from(0.34013387707001991332e0_f64) * t40784 - F::cast_from(0.42874018118069736972e-3_f64) * t40789 - F::cast_from(0.1543464652250510531e-1_f64) * t4362 * t14791 * t50511 * t10786 + F::cast_from(0.25724410870841842183e-2_f64) * t2745 * t14791 * t14494 * t14872 + F::cast_from(35.0_f64) / F::cast_from(24.0_f64) * t40792 - F::cast_from(0.18007087609589289528e-1_f64) * t51000 + F::cast_from(0.38586616306262763275e-2_f64) * t4362 * t4364 * t14676 * t2724 - F::cast_from(0.12004725073059526352e-1_f64) * t51006 + F::cast_from(0.27107389498472794074e-4_f64) * t40801 + F::cast_from(0.25724410870841842183e-1_f64) * t4362 * t14785 * t14586 * t2394 * t836 + F::cast_from(0.77173232612525526549e-1_f64) * t2745 * t51014 * t14772 * t837 + F::cast_from(0.38586616306262763276e-2_f64) * t4362 * t36833 * t50560 * t4366 - F::cast_from(0.15246000842785598467e-3_f64) * t40804 + t40810 - F::cast_from(0.12004725073059526352e-1_f64) * t40816;
    t51025
}
