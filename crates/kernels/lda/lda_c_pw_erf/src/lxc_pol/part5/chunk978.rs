//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 978/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk978<F: Float>(t339: F, t5685: F, t8495: F, t8497: F, t1064: F, t1775: F, t8510: F, t8518: F, t8524: F, t1067: F, t1765: F, t2737: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14423 = F::new(24.0) * t339 * t5685;
    let t14432 = F::new(36.0) * t8495;
    let t14433 = F::new(96.0) * t8497;
    let t14435 = t1064 * t1775;
    let t14437 = F::new(96.0) * t8510;
    let t14439 = F::new(960.0) * t8518;
    let t14440 = F::new(192.0) * t8524;
    let t14443 = t1067 * t1775;
    let t14444 = F::new(36.0) * t14443;
    let t14445 = t1765 * t2737;
    (t14423, t14432, t14433, t14435, t14437, t14439, t14440, t14444, t14445)
}
