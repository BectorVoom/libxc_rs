//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1330/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1330<F: Float>(t1358: F, t2439: F, t7274: F, t785: F, t26064: F, t3920: F, t1444: F, t4004: F, t213: F, t225: F, t25921: F, t25930: F, t25931: F, t25933: F, t25934: F, t25961: F, t27868: F, t27980: F, t46422: F, t561: F, t94574: F, t94694: F, t94700: F, t94703: F, t94705: F, t94714: F, t94716: F, t94721: F, t94726: F, t94729: F) -> F {
    let t94733 = t2439 * t785 * t7274 * t1358;
    let t94735 = t26064 * t3920;
    let t94737 = t4004 * t1444;
    let t94744 = F::new(0.32927245914677557992e-1) * t94694 + t94700 - t94703 - F::new(0.52041769129231196772e1) * t94705 * t25934 + F::new(0.65854491829355115987e0) * t213 * t94574 * t225 * t561 + F::new(0.26020884564615598386e1) * t25921 * t25961 - F::new(0.21951497276451705329e-1) * t94714 - F::new(0.52041769129231196772e1) * t25930 * t94716 * t25933 - F::new(0.26020884564615598386e1) * t25930 * t25931 * t94721 - F::new(0.34697458558045176417e-2) * t94726 - F::new(0.32927245914677557992e-1) * t94729 - F::new(0.19514881078765566038e-2) * t94733 - F::new(0.39029762157531132076e-1) * t94735 + F::new(0.52041769129231196772e1) * t25930 * t27980 * t94737 - F::new(0.26020884564615598386e1) * t27868 * t27980 * t46422;
    t94744
}
