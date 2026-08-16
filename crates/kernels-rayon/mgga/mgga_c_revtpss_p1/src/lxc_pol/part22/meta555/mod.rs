//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta555 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2381;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta555(t1269: f64, t1287: f64, t5284: f64, t17633: f64, t5458: f64, t17482: f64, t3769: f64, t3783: f64, t12713: f64, t5332: f64, t13147: f64, t487: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t17826, t17829, t17834, t17837, t17840, t17845) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2381(t1269, t1287, t5284, t17633, t5458, t17482, t3769, t3783, t12713, t5332, t13147, t487);
    (t17826, t17829, t17834, t17837, t17840, t17845)
}
