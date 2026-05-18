//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1202/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1202<F: Float>(t17557: F, t813: F, t17785: F, t21738: F, t21739: F, t21740: F, t21741: F, t21743: F, t21746: F, t21750: F, t21752: F, t21754: F, t21757: F, t21761: F, t21764: F) -> (F, F, F) {
    let t21766 = F::new(4.0) / F::new(5.0) * t17557 * t813;
    let t21767 = F::new(16.0) / F::new(45.0) * t17785;
    let t21768 = t21738 - t21739 + t21740 + t21741 + t21743 + t21746 - t21750 - t21752 - t21754 - t21757 + t21761 + t21764 + t21766 + t21767;
    (t21766, t21767, t21768)
}
