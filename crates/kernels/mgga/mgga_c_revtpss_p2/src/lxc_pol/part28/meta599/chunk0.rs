//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2075/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2075<F: Float>(t25877: F, t94889: F, t25881: F, t786: F, t94878: F, t7286: F, t4132: F, t689: F, t7242: F, t2023: F, t4075: F, t9682: F) -> (F, F, F, F, F, F) {
    let t94890 = t94889 * t25877;
    let t94891 = t94890 * t25881;
    let t94894 = t786 * t94878;
    let t94895 = t94894 * t7286;
    let t94898 = t689 * t7242 * t4132;
    let t94901 = t786 * t2023 * t4075;
    let t94902 = t94901 * t9682;
    (t94890, t94891, t94895, t94898, t94901, t94902)
}
