//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta300 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1183;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1184;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta300(t409: f64, t416: f64, t1134: f64, t3391: f64, t406: f64, t12252: f64, t12259: f64, t12261: f64, t12263: f64, t12265: f64, t12271: f64, t12275: f64, t12279: f64, t12284: f64, t12289: f64, t12292: f64, t12323: f64, t3390: f64, t3399: f64, t3407: f64, t12295: f64, t11335: f64, t281: f64, t414: f64, t1139: f64, t12322: f64, t12297: f64, t12299: f64, t12301: f64, t12303: f64, t12307: f64, t12310: f64, t12314: f64, t12317: f64, t12320: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12327, t12329, t12331, t12332, t12334) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1183(t409, t416, t1134, t3391, t406, t12252, t12259, t12261, t12263, t12265, t12271, t12275, t12279, t12284, t12289, t12292, t12323);
        let (t12343, t12344, t12346, t12347, t12351, t12354, t12356) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1184(t1134, t3390, t3399, t3407, t12295, t11335, t281, t414, t1139, t12322, t12297, t12299, t12301, t12303, t12307, t12310, t12314, t12317, t12320);
    (t12327, t12329, t12331, t12332, t12334, t12343, t12344, t12346, t12347, t12351, t12354, t12356)
}
