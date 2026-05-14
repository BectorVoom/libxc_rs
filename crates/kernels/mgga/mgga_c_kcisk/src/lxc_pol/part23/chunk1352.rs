//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1352/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1352<F: Float>(t19088: F, t52483: F, t9461: F, t32022: F, t33469: F, t3936: F, t9427: F, t1056: F, t1327: F, t5675: F, t6174: F, t5670: F, t110279: F, t110285: F, t110294: F, t113690: F, t113695: F, t113702: F, t113704: F, t113710: F, t113714: F, t32087: F, t32102: F, t33477: F, t9426: F, t9809: F) -> (F, F, F, F) {
    let t113717 = t52483 * t9461 * t19088;
    let t113719 = t32022 * t33469;
    let t113721 = t3936 * t9427;
    let t113722 = t1327 * t1056;
    let t113724 = t113721 * t5675 * t113722;
    let t113727 = t6174 * t9427;
    let t113729 = t113727 * t5670 * t113722;
    let t113732 = 0.69444444444444444446e-2 * t110279 + 0.120625e-1 * t9426 * t113690 - 0.40208333333333333335e-2 * t9426 * t113695 + 0.23280625000000000001e-2 * t32102 * t113690 - 0.69444444444444444446e-2 * t110285 + t113702 + 0.29479012345679012345e-2 * t113704 - 0.55555555555555555558e-1 * t110294 * t9809 - t113710 + 0.37037037037037037038e-1 * t32022 * t33477 + 0.73697530864197530861e-2 * t113714 + 0.11054629629629629629e-1 * t113717 + 0.61728395061728395063e-2 * t113719 - 0.13888888888888888889e-1 * t32087 * t113724 + 0.92592592592592592594e-2 * t32087 * t113729;
    (t113717, t113724, t113729, t113732)
}
