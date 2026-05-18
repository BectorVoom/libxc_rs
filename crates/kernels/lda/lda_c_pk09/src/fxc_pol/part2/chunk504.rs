//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 504/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk504<F: Float>(t1862: F, t1864: F, t1866: F, t1868: F, t2733: F, t2736: F, t2803: F, t2807: F, t1871: F, t452: F, t2795: F, t1784: F, t1786: F, t1788: F, t1790: F) -> (F, F, F, F, F) {
    let t2870 = t1862 - F::new(3.2084841915276807) * t2803 + t1864 + F::new(3.2084841915276807) * t2807 + t1866 - F::new(0.64) * t2733 + t1868 + F::new(0.64) * t2736;
    let t2871 = t2870 * t1871;
    let t2872 = t2871 * t452;
    let t2877 = t2795 * t452;
    let t2888 = t1784 - F::new(2.0) * t2803 + t1786 + F::new(2.0) * t2807 + t1788 - F::new(0.505765839233979) * t2733 + t1790 + F::new(0.505765839233979) * t2736;
    (t2870, t2871, t2872, t2877, t2888)
}
