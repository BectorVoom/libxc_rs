//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta765 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2563;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2564;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2565;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta765(t56183: f64, t2435: f64, t5057: f64, t1716: f64, t9292: f64, t12256: f64, t3617: f64, t3362: f64, t482: f64, t12268: f64, t1263: f64, t460: f64, t488: f64, t13181: f64, t1828: f64, t12627: f64, t12626: f64, t1769: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t56184, t56228) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2563(t56183, t2435, t5057);
        let (t56229, t56236) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2564(t56228, t1716, t9292);
        let (t56246, t56250, t56254, t56314, t56315, t56327, t56331) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2565(t12256, t3617, t3362, t482, t12268, t1263, t460, t488, t13181, t1828, t12627, t12626, t1769);
    (t56184, t56228, t56229, t56236, t56246, t56250, t56254, t56314, t56315, t56327, t56331)
}
