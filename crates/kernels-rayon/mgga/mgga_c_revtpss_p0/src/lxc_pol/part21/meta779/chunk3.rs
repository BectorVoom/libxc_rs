//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2777/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2777(t14746: f64, t2703: f64, t14923: f64, t14927: f64, t10811: f64, t14697: f64, t40672: f64, t828: f64, t10786: f64, t10861: f64, t14494: f64, t14586: f64, t14676: f64, t14772: f64, t14785: f64, t14791: f64, t14872: f64, t2394: f64, t2724: f64, t2745: f64, t2747: f64, t36833: f64, t40782: f64, t40784: f64, t40789: f64, t40792: f64, t40801: f64, t40804: f64, t40810: f64, t40816: f64, t4362: f64, t4364: f64, t4366: f64, t4450: f64, t50511: f64, t50560: f64, t836: f64, t837: f64) -> f64 {
    let t50982 = t2703 * t14746;
    let t51000 = t14923 * t14927;
    let t51006 = t10811 * t14697;
    let t51014 = t40672 * t828;
    let t51025 = 7.0_f64 / 48.0_f64 * t50982 - 0.51448821741683684367e-2_f64 * t4362 * t2747 * t4450 * t10861 + 0.45351183609335988443e0_f64 * t40782 + 0.34013387707001991332e0_f64 * t40784 - 0.42874018118069736972e-3_f64 * t40789 - 0.1543464652250510531e-1_f64 * t4362 * t14791 * t50511 * t10786 + 0.25724410870841842183e-2_f64 * t2745 * t14791 * t14494 * t14872 + 35.0_f64 / 24.0_f64 * t40792 - 0.18007087609589289528e-1_f64 * t51000 + 0.38586616306262763275e-2_f64 * t4362 * t4364 * t14676 * t2724 - 0.12004725073059526352e-1_f64 * t51006 + 0.27107389498472794074e-4_f64 * t40801 + 0.25724410870841842183e-1_f64 * t4362 * t14785 * t14586 * t2394 * t836 + 0.77173232612525526549e-1_f64 * t2745 * t51014 * t14772 * t837 + 0.38586616306262763276e-2_f64 * t4362 * t36833 * t50560 * t4366 - 0.15246000842785598467e-3_f64 * t40804 + t40810 - 0.12004725073059526352e-1_f64 * t40816;
    t51025
}
