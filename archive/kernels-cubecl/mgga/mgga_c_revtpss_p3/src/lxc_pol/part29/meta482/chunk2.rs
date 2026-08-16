//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1765/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1765<F: Float>(t1426: F, t27836: F, t7063: F, t786: F, t5629: F, t7271: F, t1885: F, t26024: F, t25972: F, t5622: F, t1889: F, t25978: F) -> (F, F, F, F, F, F, F) {
    let t27883 = t27836 * t1426;
    let t27884 = t7063 * t27883;
    let t27899 = t786 * t27883;
    let t27919 = t7271 * t5629;
    let t27921 = t26024 * t1885;
    let t27924 = t25972 * t5622;
    let t27926 = t25978 * t1889;
    (t27883, t27884, t27899, t27919, t27921, t27924, t27926)
}
