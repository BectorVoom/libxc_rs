//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1248/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1248<F: Float>(t37443: F, t37444: F, t37448: F, t37452: F, t40342: F, t40346: F, t42876: F, t42881: F, t42885: F, t42889: F, t42893: F, t42897: F, t42900: F, t42904: F, t42908: F) -> F {
    let t43867 = t37443 - t42876 + t40342 - t40346 + t42881 - t42885 - t42889 + t42893 - t42897 + t42900 + F::new(0.30487649791575028314e-3) * t37444 - t37448 + t42904 - t37452 - t42908;
    t43867
}
