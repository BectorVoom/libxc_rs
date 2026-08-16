//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1102/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1102<F: Float>(t30: F, t892: F, t14685: F, t1941: F, t241: F, t25260: F, t820: F, t1955: F, t7057: F, t11064: F, t33: F, t7283: F) -> (F, F, F, F, F, F, F, F) {
    let t27159 = t892 * t30;
    let t27221 = t1941 * t14685;
    let t27261 = t820 * t25260 * t241;
    let t27353 = t1955 * t7057;
    let t27383 = t11064 * t30;
    let t27763 = t892 * t33;
    let t27799 = t11064 * t33;
    let t27868 = t1955 * t7283;
    (t27159, t27221, t27261, t27353, t27383, t27763, t27799, t27868)
}
