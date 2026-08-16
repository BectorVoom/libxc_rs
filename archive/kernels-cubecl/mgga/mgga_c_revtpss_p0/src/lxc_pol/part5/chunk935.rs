//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 935/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk935<F: Float>(t1868: F, t5532: F, t3854: F, t3859: F, t3862: F, t3865: F, t3867: F, t3871: F, t3873: F, t4027: F, t4035: F, t4037: F, t4042: F, t4139: F, t6827: F, t6828: F) -> F {
    let t6930 = t5532 * t1868;
    let t6933 = F::cast_from(6.0_f64) * t4139 * t6930 + t3854 + t3859 + t3862 + t3865 - t3867 + t3871 + t3873 - t4027 - t4035 - t4037 + t4042 + t6827 - t6828;
    t6933
}
