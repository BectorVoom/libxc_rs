//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 780/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk780<F: Float>(t3951: F, t547: F, t807: F, t2700: F, t535: F, t1369: F, t794: F, t1372: F, t124: F, t3889: F, t800: F, t2453: F, t546: F) -> (F, F, F, F, F, F, F) {
    let t3952 = t547 * t3951;
    let t3953 = t807 * t3952;
    let t3956 = F::cast_from(35.0_f64) / F::cast_from(432.0_f64) * t2700 * t535;
    let t3957 = t794 * t1369;
    let t3958 = t3957 * t1372;
    let t3960 = t124 * t3889;
    let t3961 = t800 * t3960;
    let t3964 = t2453 * t546;
    (t3952, t3953, t3956, t3957, t3958, t3961, t3964)
}
