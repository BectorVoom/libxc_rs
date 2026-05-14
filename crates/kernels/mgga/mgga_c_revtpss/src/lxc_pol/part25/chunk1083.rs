//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1083/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1083<F: Float>(t1962: F, t198: F, t205: F, t30: F, t892: F, t14685: F, t1941: F, t241: F, t25260: F, t820: F, t1955: F, t7057: F, t1949: F, t2718: F, t1993: F, t11064: F) -> (F, F, F, F, F, F, F, F) {
    let t27158 = t198 * t205 * t1962;
    let t27159 = t892 * t30;
    let t27221 = t1941 * t14685;
    let t27261 = t820 * t25260 * t241;
    let t27353 = t1955 * t7057;
    let t27357 = t2718 * t1949;
    let t27382 = t198 * t1993;
    let t27383 = t11064 * t30;
    (t27158, t27159, t27221, t27261, t27353, t27357, t27382, t27383)
}
