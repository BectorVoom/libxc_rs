//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta626 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2160;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2161;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta626(t16060: f64, t3865: f64, t1831: f64, t40292: f64, t12345: f64, t5314: f64, t40018: f64, t5223: f64, t12282: f64, t5234: f64, t12189: f64, t5227: f64, t40281: f64, t5303: f64, t5247: f64, t820: f64, t12250: f64, t1824: f64, t3789: f64, t12384: f64, t5293: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t53906, t53918, t53920, t53928, t53945, t53984) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2160(t16060, t3865, t1831, t40292, t12345, t5314, t40018, t5223, t12282, t5234, t12189, t5227);
        let (t53985, t53998, t54013, t54014, t54023, t54042, t54047) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2161(t53984, t40281, t5303, t5247, t820, t12250, t1824, t16060, t3789, t12384, t5234, t5293);
    (t53906, t53918, t53920, t53928, t53945, t53985, t53998, t54013, t54014, t54023, t54042, t54047)
}
