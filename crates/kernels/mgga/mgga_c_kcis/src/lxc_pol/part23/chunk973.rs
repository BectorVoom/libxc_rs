//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 973/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk973<F: Float>(t4463: F, t7984: F, t6176: F, t2259: F, t26971: F, t2257: F, t7964: F, t7974: F, t3801: F, t7979: F, t1600: F, t27482: F, t27432: F, t27462: F, t27465: F, t27477: F, t27480: F, t27607: F, t27617: F, t7968: F, t7978: F, t7981: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27647 = t7984 * t4463;
    let t27648 = t6176 * t27647;
    let t27651 = t26971 * t2259;
    let t27653 = 0.7722800925925925926e-4 * t2257 * t27651;
    let t27654 = t7964 * t7974;
    let t27664 = t7979 * t3801;
    let t27665 = t1600 * t27664;
    let t27668 = 0.38691203703703703703e-3 * t27482;
    let t27669 = 0.15476481481481481481e-2 * t27432 + 0.34752604166666666667e-3 * t7978 * t27648 + t27653 - 0.23168402777777777778e-3 * t27654 + 0.23214722222222222222e-2 * t27462 + 0.17411041666666666666e-2 * t27465 - 0.34822083333333333332e-2 * t27477 + 0.23214722222222222222e-2 * t27480 - 0.92754700520833333334e-4 * t7968 * t27617 - 0.23168402777777777778e-3 * t27607 * t7981 + 0.23168402777777777778e-3 * t7978 * t27665 - t27668;
    (t27647, t27648, t27651, t27653, t27654, t27664, t27665, t27668, t27669)
}
