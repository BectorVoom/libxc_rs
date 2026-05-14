//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1070/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1070<F: Float>(t10196: F, t3818: F, t1083: F, t30091: F, t30094: F, t30096: F, t30098: F, t30103: F, t30105: F, t31906: F, t31909: F, t31912: F, t31915: F, t31919: F, t31922: F, t3359: F) -> (F,) {
    let t31924 = 0.7588001769513639893e-1 * t3818 * t10196;
    let t31925 = -t31906 - t31909 + t31912 + t31915 - 0.7588001769513639893e-1 * t1083 * t3359 + t30091 + t30094 + t30096 - t31919 + t30098 + t30103 - t30105 + t31922 - t31924;
    (t31925,)
}
