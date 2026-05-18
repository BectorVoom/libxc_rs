//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 887/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk887<F: Float>(t124: F, t22813: F, t800: F, t1883: F, t22079: F, t5673: F, t1872: F, t6816: F, t22046: F, t3936: F, t6869: F, t543: F, t6836: F) -> (F, F, F, F, F) {
    let t22876 = t124 * t22813;
    let t22877 = t800 * t22876;
    let t22881 = t5673 * t22079 * t1883;
    let t22886 = t800 * t1872 * t6816;
    let t22890 = t3936 * t22046 * t6869;
    let t22893 = t543 * t6836;
    (t22877, t22881, t22886, t22890, t22893)
}
