//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 896/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk896<F: Float>(t8821: F, t8823: F, t8826: F, t8831: F, t8832: F, t8835: F, t8839: F, t8844: F, t8846: F, t8853: F, t8854: F, t8858: F, t2118: F, t3106: F, t3074: F, t745: F, t857: F, t858: F) -> (F, F, F, F) {
    let t8859 = -t8821 + t8823 + t8826 + t8831 + t8832 + t8835 - t8839 + t8844 - t8846 - t8853 - t8854 + t8858;
    let t8860 = t2118 * t3106;
    let t8861 = t3074 * t8860;
    let t8863 = t857 * t858 * t745;
    (t8859, t8860, t8861, t8863)
}
