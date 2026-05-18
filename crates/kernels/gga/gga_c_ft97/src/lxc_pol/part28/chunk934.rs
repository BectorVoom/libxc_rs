//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 934/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk934<F: Float>(t32075: F, t637: F, t1307: F, t7242: F, t32057: F, t32059: F, t32063: F, t1317: F, t32326: F, t376: F, t2253: F, t5664: F) -> (F, F, F, F, F) {
    let t136188 = t637 * t32075;
    let t136189 = t7242 * t1307;
    let t136226 = t32057 * t32063 * t32059;
    let t136229 = t1317 * t376 * t32326;
    let t136240 = t5664 * t2253;
    (t136188, t136189, t136226, t136229, t136240)
}
