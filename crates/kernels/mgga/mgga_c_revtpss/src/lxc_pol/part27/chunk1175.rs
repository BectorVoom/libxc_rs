//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1175/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1175<F: Float>(t1962: F, t198: F, t205: F, t30: F, t892: F, t14685: F, t1941: F, t241: F, t25260: F, t820: F, t1955: F, t7057: F) -> (F, F, F, F, F) {
    let t27158 = t198 * t205 * t1962;
    let t27159 = t892 * t30;
    let t27221 = t1941 * t14685;
    let t27261 = t820 * t25260 * t241;
    let t27353 = t1955 * t7057;
    (t27158, t27159, t27221, t27261, t27353)
}
