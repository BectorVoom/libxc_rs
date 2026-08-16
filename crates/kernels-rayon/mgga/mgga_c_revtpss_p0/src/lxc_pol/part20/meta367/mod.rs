//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta367 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1340;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1341;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta367(t10326: f64, t706: f64, t750: f64, t2523: f64, t9419: f64, t40093: f64, t40095: f64, t40099: f64, t40103: f64, t40106: f64, t40109: f64, t40111: f64, t40115: f64, t40117: f64, t10558: f64, t72: f64, t757: f64, t10573: f64, t2619: f64, t2598: f64, t9321: f64, t760: f64, t9387: f64, t2495: f64, t39875: f64, t9367: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40120, t40122, t40123) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1340(t10326, t706, t750, t2523, t9419, t40093, t40095, t40099, t40103, t40106, t40109, t40111, t40115, t40117);
        let (t40126, t40128, t40129, t40131, t40133, t40135) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1341(t10558, t72, t757, t10573, t2619, t2598, t9321, t760, t2523, t9387, t2495, t39875, t9367);
    (t40120, t40122, t40123, t40126, t40128, t40129, t40131, t40133, t40135)
}
