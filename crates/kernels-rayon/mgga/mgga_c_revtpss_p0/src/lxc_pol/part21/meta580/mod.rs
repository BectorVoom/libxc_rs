//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta580 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2288;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2289;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta580(t17948: f64, t460: f64, t12050: f64, t3601: f64, t471: f64, t17710: f64, t1204: f64, t5462: f64, t3754: f64, t5219: f64, t1234: f64, t12699: f64, t12709: f64, t12717: f64, t12723: f64, t1285: f64, t17331: f64, t1770: f64, t17917: f64, t17921: f64, t17934: f64, t17941: f64, t17945: f64, t1822: f64, t3670: f64, t3746: f64, t3756: f64, t3770: f64, t3774: f64, t3778: f64, t3787: f64, t490: f64, t5436: f64, t5446: f64, t5466: f64, t5470: f64, t5491: f64, t17186: f64, t17859: f64, t17912: f64, t1277: f64, t1828: f64, t3738: f64, t13182: f64, t3566: f64, t488: f64, t1276: f64, t1774: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17949, t17951, t17952, t17955, t17958, t17961) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2288(t17948, t460, t12050, t3601, t471, t17710, t1204, t5462, t3754, t5219, t1234, t12699, t12709, t12717, t12723, t1285, t17331, t1770, t17917, t17921, t17934, t17941, t17945, t1822, t3670, t3746, t3756, t3770, t3774, t3778, t3787, t490, t5436, t5446, t5466, t5470, t5491);
        let (t17963, t17964, t17968, t17973, t17974) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2289(t17186, t17859, t17912, t17961, t1277, t1828, t3738, t13182, t3566, t488, t1276, t1774);
    (t17949, t17951, t17952, t17955, t17958, t17963, t17964, t17968, t17973, t17974)
}
