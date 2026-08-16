//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta567 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2410;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2411;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta567(t4343: f64, t4542: f64, t2404: f64, t5966: f64, t14613: f64, t162: f64, t4403: f64, t14312: f64, t5940: f64, t705: f64, t707: f64, t10605: f64, t6002: f64, t2411: f64, t6079: f64, t10446: f64, t5819: f64, t2375: f64, t5825: f64, t13309: f64, t13310: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18253, t18256, t18259, t18261, t18262, t18263, t18265, t18267) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2410(t4343, t4542, t2404, t5966, t14613, t162, t4403, t14312, t5940, t705, t707, t10605, t6002);
        let (t18268, t18272, t18277, t18280) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2411(t2411, t6079, t10446, t5819, t2375, t5825, t13309, t13310);
    (t18253, t18256, t18259, t18261, t18262, t18263, t18265, t18267, t18268, t18272, t18277, t18280)
}
