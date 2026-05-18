//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 930/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk930<F: Float>(t153: F, t274: F, t8798: F, t242: F, t4130: F, t1155: F, t632: F, t4137: F, t1198: F, t1426: F, t1159: F, t646: F) -> (F, F, F, F, F, F) {
    let t11006 = F::new(19.1926369973667) * t153 * t8798 * t274;
    let t11007 = t4130 * t242;
    let t11010 = F::new(2.0103076928521055) * t1155 * t632;
    let t11012 = F::new(2.0103076928521055) * t4137 * t242;
    let t11020 = t1198 * t1426;
    let t11022 = t1159 * t646;
    (t11006, t11007, t11010, t11012, t11020, t11022)
}
