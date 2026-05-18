//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 868/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk868<F: Float>(t2206: F, t3195: F, t1114: F, t6677: F, t1134: F, t814: F, t858: F, t3065: F, t328: F, t6643: F, t824: F, t874: F, t8884: F) -> (F, F, F, F, F, F) {
    let t8973 = F::new(7.0) / F::new(72.0) * t2206 * t3195;
    let t8978 = t1114 * t6677;
    let t8981 = t1134 * t814;
    let t8982 = t858 * t8981;
    let t8983 = t3065 * t8982;
    let t8986 = t6643 * t328;
    let t8987 = t824 * t8986;
    let t8989 = t8884 * t874;
    (t8973, t8978, t8981, t8983, t8987, t8989)
}
