//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1083/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1083<F: Float>(t13625: F, t8717: F, t25082: F, t1450: F, t3889: F, t7237: F, t2014: F, t7235: F, t7316: F, t2242: F, t607: F, t640: F, t644: F, t77: F) -> (F, F, F, F, F, F, F, F) {
    let t25083 = t8717 * t13625;
    let t25085 = F::cast_from(6.0_f64) * t25082 * t25083;
    let t25089 = t1450 * t3889;
    let t25090 = t7237 * t25089;
    let t25092 = F::cast_from(3.0_f64) * t2014 * t25090;
    let t25095 = F::cast_from(2.0_f64) * t7235 * t7316;
    let t25102 = t2242 * t607;
    let t25110 = t77 * t640 * t644;
    (t25083, t25085, t25089, t25090, t25092, t25095, t25102, t25110)
}
