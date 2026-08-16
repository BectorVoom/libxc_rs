//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1333/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1333(t26685: f64, t95890: f64, t1020: f64, t8047: f64, t92910: f64, t1250: f64, t43526: f64, t42972: f64, t7690: f64, t96356: f64, t10463: f64, t26784: f64, t26823: f64, t27808: f64, t27915: f64, t27936: f64, t7687: f64, t7693: f64, t7703: f64, t7711: f64, t8034: f64, t8042: f64, t93718: f64, t96372: f64, t982: f64, t990: f64) -> (f64, f64) {
    let t96508 = 0.18550940104166666667e-3_f64 * t26685 * t95890;
    let t96510 = t1020 * t92910 * t8047;
    let t96522 = t43526 * t1250;
    let t96527 = t42972 * t1250;
    let t96534 = t7690 * t96356;
    let t96536 = -t96508 + 0.1621345679012345679e-1_f64 * t96510 + 0.99024918276041666665e-4_f64 * t10463 * t982 * t990 * t27808 - 0.69505208333333333333e-3_f64 * t7703 * t96372 + 0.13901041666666666667e-2_f64 * t27936 * t7711 + 0.13901041666666666667e-2_f64 * t27936 * t7693 + 0.18550940104166666667e-3_f64 * t96522 * t7693 + 0.69505208333333333333e-3_f64 * t26823 * t8042 - 0.185671721767578125e-4_f64 * t96527 * t26784 + 0.92754700520833333333e-4_f64 * t93718 * t8034 + 0.13901041666666666667e-2_f64 * t7687 * t27915 + 0.61836467013888888888e-4_f64 * t96534;
    (t96510, t96536)
}
