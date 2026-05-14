//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 947/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk947<F: Float>(t11749: F, t1818: F, t2052: F, t2795: F, t7080: F, t11096: F, t11711: F, t11715: F, t11717: F, t11721: F, t11723: F, t11733: F, t1748: F, t2032: F, t2071: F, t2085: F, t2091: F, t2783: F, t2796: F, t453: F, t6288: F) -> (F, F) {
    let t11750 = t11749 * t1818;
    let t11751 = t11750 * t2052;
    let t11754 = t2795 * t7080;
    let t11759 = t11711 * t11096 / 3.0 + t11715 * t11717 / 6.0 + t11721 / 6.0 + t11723 * t2032 / 6.0 - t2796 * t6288 / 6.0 + t2085 * t2783 / 6.0 + t2091 * t2783 / 6.0 + t453 * t11733 / 6.0 - t11751 * t1748 / 6.0 - t11754 * t1748 / 6.0 - t2071 * t2783 / 6.0;
    (t11750, t11759)
}
