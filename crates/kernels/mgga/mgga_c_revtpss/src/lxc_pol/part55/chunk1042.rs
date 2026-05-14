//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1042/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1042<F: Float>(t32206: F, t32211: F, t5673: F, t5774: F, t121024: F, t33964: F, t125668: F, t3936: F, t121057: F, t33922: F, t32283: F, t33943: F, t1401: F, t32194: F, t32195: F, t5659: F) -> (F, F, F, F, F, F) {
    let t125771 = t32206 * t5673 * t32211 * t5774;
    let t125775 = t121024 * t33964;
    let t125780 = t32206 * t3936 * t32211 * t125668;
    let t125782 = t121057 * t33922;
    let t125784 = t33943 * t32283;
    let t125785 = t125784 * t1401;
    let t125793 = t32194 * t5673 * t32195 * t5659;
    (t125771, t125775, t125780, t125782, t125785, t125793)
}
