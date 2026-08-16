//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1193/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1193(t31918: f64, t1063: f64, t2765: f64, t30200: f64, t10196: f64, t3818: f64, t1083: f64, t30091: f64, t30094: f64, t30096: f64, t30098: f64, t30103: f64, t30105: f64, t31906: f64, t31909: f64, t31912: f64, t31915: f64, t3359: f64) -> f64 {
    let t31919 = 0.31616674039640166222e-2_f64 * t31918;
    let t31922 = 0.17073003981405689759e0_f64 * t1063 * t2765 * t30200;
    let t31924 = 0.7588001769513639893e-1_f64 * t3818 * t10196;
    let t31925 = -t31906 - t31909 + t31912 + t31915 - 0.7588001769513639893e-1_f64 * t1083 * t3359 + t30091 + t30094 + t30096 - t31919 + t30098 + t30103 - t30105 + t31922 - t31924;
    t31925
}
