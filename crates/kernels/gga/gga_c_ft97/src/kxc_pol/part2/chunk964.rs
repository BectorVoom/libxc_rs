//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 964/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk964<F: Float>(t13352: F, t4199: F, t10603: F, t14671: F, t13346: F, t4206: F, t14648: F, t2771: F, t14628: F, t13296: F, t13301: F, t14664: F) -> (F, F, F, F, F, F, F, F) {
    let t14971 = t4199 * t13352;
    let t14974 = t10603 * t14671;
    let t14977 = t4206 * t13346;
    let t14980 = t2771 * t14648;
    let t14983 = t2771 * t14628;
    let t14986 = t4206 * t13296;
    let t14989 = t4206 * t13301;
    let t14992 = t2771 * t14664;
    (t14971, t14974, t14977, t14980, t14983, t14986, t14989, t14992)
}
