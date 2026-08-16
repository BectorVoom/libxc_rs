//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta926 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3149;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3150;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta926(t17351: f64, t17354: f64, t56756: f64, t3588: f64, t3611: f64, t12904: f64, t5293: f64, t12959: f64, t17569: f64, t11262: f64, t1261: f64, t5269: f64, t17236: f64, t3172: f64, t17540: f64, t3711: f64, t12956: f64, t17209: f64, t17198: f64, t12773: f64, t17605: f64, t17557: f64, t17535: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t56758, t56760, t56785, t56787, t56790) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3149(t17351, t17354, t56756, t3588, t3611, t12904, t5293, t12959, t17569, t11262, t1261, t5269);
        let (t56793, t56796, t56798, t56812, t56835, t56838, t56853) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3150(t1261, t17236, t3172, t17540, t3711, t12956, t17209, t17198, t12773, t17605, t17557, t17535);
    (t56758, t56760, t56785, t56787, t56790, t56793, t56796, t56798, t56812, t56835, t56838, t56853)
}
