//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 795/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk795<F: Float>(t378: F, t554: F, t32179: F, t5821: F, t5824: F, t53: F, t5555: F, t129: F, t5551: F, t135: F, t32213: F, t52: F, t7182: F) -> (F, F, F, F, F, F, F, F) {
    let t32775 = t378 * t554;
    let t32782 = F::new(0.30209702213418583705e-1) * t5821 * t32179;
    let t32786 = F::new(0.30209702213418583705e-1) * t5824 * t32179;
    let t32791 = t53 * t5555;
    let t32795 = t129 * t5551;
    let t32796 = t32213 * t135;
    let t32797 = t32795 * t32796;
    let t32803 = t52 * t7182 * t554;
    (t32775, t32782, t32786, t32791, t32795, t32796, t32797, t32803)
}
