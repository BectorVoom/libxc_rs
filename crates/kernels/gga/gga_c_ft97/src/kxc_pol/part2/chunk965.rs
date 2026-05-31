//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 965/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk965<F: Float>(t10613: F, t14653: F, t1775: F, t4215: F, t14660: F, t2771: F, t14889: F, t192: F, t852: F, t302: F, t668: F, t683: F) -> (F, F, F, F, F) {
    let t14995 = t10613 * t14653;
    let t14999 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1775 * t4215;
    let t15000 = t2771 * t14660;
    let t15004 = t192 * t852 * t14889;
    let t15007 = t683 * t302 * t668;
    (t14995, t14999, t15000, t15004, t15007)
}
