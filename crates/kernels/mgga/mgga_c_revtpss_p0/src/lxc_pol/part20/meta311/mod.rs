//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta311 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1213;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1214;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta311<F: Float>(t12705: F, t5480: F, t3555: F, t3754: F, t1248: F, t3153: F, t3588: F, t5464: F, t3566: F, t3568: F, t1287: F, t1269: F, t1284: F, t1209: F, t3584: F, t12233: F, t12240: F, t12242: F, t12245: F, t12251: F, t12360: F, t12363: F, t12573: F, t12575: F, t12577: F, t12598: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t12706, t12709, t12712, t12714, t12717, t12718, t12719, t12722) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1213::<F>(t12705, t5480, t3555, t3754, t1248, t3153, t3588, t5464, t3566, t3568, t1287, t1269, t1284);
        let (t12723, t12726, t12727, t12730) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1214::<F>(t1209, t12722, t1248, t3584, t1287, t12233, t12240, t12242, t12245, t12251, t12360, t12363, t12573, t12575, t12577, t12598);
    (t12706, t12709, t12712, t12714, t12717, t12718, t12719, t12722, t12723, t12726, t12727, t12730)
}
