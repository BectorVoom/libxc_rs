//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta311 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1213;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1214;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta311(t12705: f64, t5480: f64, t3555: f64, t3754: f64, t1248: f64, t3153: f64, t3588: f64, t5464: f64, t3566: f64, t3568: f64, t1287: f64, t1269: f64, t1284: f64, t1209: f64, t3584: f64, t12233: f64, t12240: f64, t12242: f64, t12245: f64, t12251: f64, t12360: f64, t12363: f64, t12573: f64, t12575: f64, t12577: f64, t12598: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12706, t12709, t12712, t12714, t12717, t12718, t12719, t12722) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1213(t12705, t5480, t3555, t3754, t1248, t3153, t3588, t5464, t3566, t3568, t1287, t1269, t1284);
        let (t12723, t12726, t12727, t12730) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1214(t1209, t12722, t1248, t3584, t1287, t12233, t12240, t12242, t12245, t12251, t12360, t12363, t12573, t12575, t12577, t12598);
    (t12706, t12709, t12712, t12714, t12717, t12718, t12719, t12722, t12723, t12726, t12727, t12730)
}
