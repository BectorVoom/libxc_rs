//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 708/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk708<F: Float>(t6325: F, t6547: F, t6464: F, t1783: F, t1793: F, t747: F, t1847: F, t6805: F, t1468: F, t523: F, t1747: F, t6302: F) -> (F, F, F, F, F, F) {
    let t6889 = F::cast_from(0.03016988933062603_f64) * t6325;
    let t6890 = F::cast_from(0.025208232546211785_f64) * t6547;
    let t6895 = F::cast_from(0.010056629776875343_f64) * t6464;
    let t6907 = t1783 * t747 * t1793;
    let t6911 = t1847 * t6805;
    let t6921 = t523 * t1468;
    let t6922 = t6921 * t1747;
    let t6924 = F::cast_from(37.27051603526593_f64) * t6922 * t6302;
    (t6889, t6890, t6895, t6907, t6911, t6924)
}
