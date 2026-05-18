//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1333/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1333<F: Float>(t26685: F, t95890: F, t1020: F, t8047: F, t92910: F, t1250: F, t43526: F, t42972: F, t7690: F, t96356: F, t10463: F, t26784: F, t26823: F, t27808: F, t27915: F, t27936: F, t7687: F, t7693: F, t7703: F, t7711: F, t8034: F, t8042: F, t93718: F, t96372: F, t982: F, t990: F) -> (F, F) {
    let t96508 = F::new(0.18550940104166666667e-3) * t26685 * t95890;
    let t96510 = t1020 * t92910 * t8047;
    let t96522 = t43526 * t1250;
    let t96527 = t42972 * t1250;
    let t96534 = t7690 * t96356;
    let t96536 = -t96508 + F::new(0.1621345679012345679e-1) * t96510 + F::new(0.99024918276041666665e-4) * t10463 * t982 * t990 * t27808 - F::new(0.69505208333333333333e-3) * t7703 * t96372 + F::new(0.13901041666666666667e-2) * t27936 * t7711 + F::new(0.13901041666666666667e-2) * t27936 * t7693 + F::new(0.18550940104166666667e-3) * t96522 * t7693 + F::new(0.69505208333333333333e-3) * t26823 * t8042 - F::new(0.185671721767578125e-4) * t96527 * t26784 + F::new(0.92754700520833333333e-4) * t93718 * t8034 + F::new(0.13901041666666666667e-2) * t7687 * t27915 + F::new(0.61836467013888888888e-4) * t96534;
    (t96510, t96536)
}
