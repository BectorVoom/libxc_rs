//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1190/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1190<F: Float>(t2322: F, t25851: F, t4254: F, t1310: F, t25832: F, t651: F, t1962: F, t41154: F, t11061: F, t30: F, t27383: F, t50066: F) -> (F, F, F, F, F, F) {
    let t92731 = F::cast_from(6.0_f64) * t2322 * t25851;
    let t92733 = F::cast_from(6.0_f64) * t4254 * t25851;
    let t92736 = F::cast_from(6.0_f64) * t651 * t1310 * t25832;
    let t92742 = t1962 * t41154;
    let t92743 = t30 * t11061;
    let t92747 = t27383 * t50066;
    (t92731, t92733, t92736, t92742, t92743, t92747)
}
