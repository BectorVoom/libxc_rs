//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta954 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3197;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3198;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta954(t1269: f64, t13126: f64, t460: f64, t13147: f64, t1770: f64, t1204: f64, t17852: f64, t1209: f64, t1284: f64, t5412: f64, t17845: f64, t17306: f64, t3754: f64, t1774: f64, t487: f64, t17807: f64, t3727: f64, t5219: f64, t2246: f64, t4171: f64, t10308: f64, t1466: f64, t13267: f64, t602: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t59945, t59948, t59987, t60008, t60013, t60019) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3197(t1269, t13126, t460, t13147, t1770, t1204, t17852, t1209, t1284, t5412, t17845, t17306, t3754);
        let (t60037, t60087, t60106, t60221, t60224, t60248) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3198(t1774, t487, t1209, t17807, t3727, t5219, t2246, t4171, t10308, t1466, t13267, t602);
    (t59945, t59948, t59987, t60008, t60013, t60019, t60037, t60087, t60106, t60221, t60224, t60248)
}
