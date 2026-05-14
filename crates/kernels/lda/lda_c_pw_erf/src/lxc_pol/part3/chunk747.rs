//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 747/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk747<F: Float>(t1710: F, t770: F, t1712: F, t774: F, t1878: F, t450: F, t1724: F, t155: F, t436: F, t1870: F, t1872: F, t1832: F, t411: F, t1568: F, t756: F, t1697: F, t9: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5621 = t770 * t1710;
    let t5630 = t774 * t1712;
    let t5633 = t1878 * t450;
    let t5636 = t774 * t1724;
    let t5639 = t155 * t436;
    let t5641 = t1870 * t5639 * t1872;
    let t5643 = t1832 * t411;
    let t5647 = t756 * t1568;
    let t5651 = t9 * t1697;
    (t5621, t5630, t5633, t5636, t5639, t5641, t5643, t5647, t5651)
}
