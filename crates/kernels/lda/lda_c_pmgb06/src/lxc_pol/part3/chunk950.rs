//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 950/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk950<F: Float>(t8781: F, t8785: F, t1105: F, t2160: F, t8738: F, t8743: F, t8746: F, t8749: F, t8751: F, t8755: F, t8759: F, t8760: F, t8762: F, t8769: F, t8774: F, t8779: F, t8787: F, t8794: F) -> F {
    let t11132 = F::new(960.0) * t8781;
    let t11133 = F::new(192.0) * t8785;
    let t11135 = t1105 * t2160;
    let t11136 = F::new(36.0) * t11135;
    let t11137 = -F::new(10.526802520742363) * t8738 - t8743 + t8746 - F::new(24.0) * t8749 - F::new(4.0) * t8751 - t8755 - t8759 + F::new(10.526802520742363) * t8760 - F::new(155.84273195113317) * t8762 + t8769 - t8774 + t8779 - t11132 + t11133 - F::new(0.0017090684152272775) * t8787 - t8794 + t11136;
    t11137
}
