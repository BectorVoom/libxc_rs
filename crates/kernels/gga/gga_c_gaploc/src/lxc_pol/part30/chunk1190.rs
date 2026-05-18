//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1190/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1190<F: Float>(t31918: F, t1063: F, t2765: F, t30200: F, t10196: F, t3818: F, t1083: F, t30091: F, t30094: F, t30096: F, t30098: F, t30103: F, t30105: F, t31906: F, t31909: F, t31912: F, t31915: F, t3359: F) -> F {
    let t31919 = F::new(0.31616674039640166222e-2) * t31918;
    let t31922 = F::new(0.17073003981405689759e0) * t1063 * t2765 * t30200;
    let t31924 = F::new(0.7588001769513639893e-1) * t3818 * t10196;
    let t31925 = -t31906 - t31909 + t31912 + t31915 - F::new(0.7588001769513639893e-1) * t1083 * t3359 + t30091 + t30094 + t30096 - t31919 + t30098 + t30103 - t30105 + t31922 - t31924;
    t31925
}
