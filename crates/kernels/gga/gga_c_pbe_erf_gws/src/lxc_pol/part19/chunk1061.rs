//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1061/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1061<F: Float>(t11912: F, t11363: F, t6384: F, t904: F, t11889: F, t2300: F, t2206: F, t3799: F, t11583: F, t337: F, t6560: F, t2146: F) -> (F, F, F, F, F, F) {
    let t11913 = F::new(7.0) / F::new(288.0) * t11912;
    let t11915 = t6384 * t904 * t11363;
    let t11919 = t2300 * t904 * t11889;
    let t11922 = t2206 * t3799;
    let t11923 = F::new(7.0) / F::new(48.0) * t11922;
    let t11924 = t337 * t11583;
    let t11925 = t6560 * t11924;
    let t11927 = t2146 * t11925 / F::new(16.0);
    (t11913, t11915, t11919, t11923, t11924, t11927)
}
