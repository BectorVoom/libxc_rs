//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1168/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1168<F: Float>(t460: F, t7627: F, t2142: F, t3555: F, t1294: F, t7644: F, t7652: F, t1204: F, t1209: F, t26936: F, t1214: F, t7637: F) -> (F, F, F, F, F, F, F) {
    let t27008 = t460 * t7627;
    let t27011 = t3555 * t2142;
    let t27015 = t7652 * t7644 * t1294;
    let t27020 = t1204 * t2142;
    let t27025 = t1209 * t26936;
    let t27028 = t7627 * t1214;
    let t27029 = t7637 * t27028;
    (t27008, t27011, t27015, t27020, t27025, t27028, t27029)
}
