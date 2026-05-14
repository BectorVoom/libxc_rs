//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 693/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk693<F: Float>(t13346: F, t4206: F, t14648: F, t2771: F, t14628: F, t13296: F, t13301: F, t14664: F, t10613: F, t14653: F, t1775: F, t4215: F, t14660: F, t14889: F, t192: F, t852: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t14977 = t4206 * t13346;
    let t14980 = t2771 * t14648;
    let t14983 = t2771 * t14628;
    let t14986 = t4206 * t13296;
    let t14989 = t4206 * t13301;
    let t14992 = t2771 * t14664;
    let t14995 = t10613 * t14653;
    let t14999 = 2.0 / 9.0 * t1775 * t4215;
    let t15000 = t2771 * t14660;
    let t15004 = t192 * t852 * t14889;
    (t14977, t14980, t14983, t14986, t14989, t14992, t14995, t14999, t15000, t15004)
}
