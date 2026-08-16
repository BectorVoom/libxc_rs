//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2656/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2656(t16155: f64, t3866: f64, t1827: f64, t40123: f64, t1824: f64, t3850: f64, t16060: f64, t3802: f64, t1799: f64, t1340: f64, t53909: f64, t12255: f64, t12305: f64, t12336: f64, t1307: f64, t1354: f64, t1363: f64, t16018: f64, t16150: f64, t16217: f64, t16224: f64, t16225: f64, t16305: f64, t16306: f64, t3783: f64, t3803: f64, t3807: f64, t3809: f64, t3851: f64, t3870: f64, t5240: f64, t5246: f64, t5248: f64, t5249: f64, t5310: f64, t54013: f64, t820: f64) -> f64 {
    let t54138 = t3866 * t16155;
    let t54151 = t40123 * t1827;
    let t54153 = t1824 * t3850;
    let t54162 = t16060 * t3802;
    let t54165 = t1799 * t3850;
    let t54178 = t53909 * t1340;
    let t54183 = -35.0_f64 / 384.0_f64 * t54138 + 5.0_f64 / 128.0_f64 * t3783 * t16150 + 5.0_f64 / 256.0_f64 * t1363 * t3870 * t820 * t16018 * t1307 + 5.0_f64 / 256.0_f64 * t12336 * t5310 + 5.0_f64 / 256.0_f64 * t5240 * t12305 + 595.0_f64 / 10368.0_f64 * t54151 + t3803 * t16305 * t54153 * t3807 / 256.0_f64 + 7.0_f64 / 1536.0_f64 * t5246 * t5248 * t5249 * t12255 + t54162 * t3809 / 128.0_f64 - 5.0_f64 / 256.0_f64 * t3803 * t16224 * t54165 * t3807 + t3803 * t16305 * t16225 * t3851 / 256.0_f64 - t3803 * t54013 * t16306 * t3851 / 1024.0_f64 - t54178 * t1354 / 1024.0_f64 - 15.0_f64 / 128.0_f64 * t3783 * t16217;
    t54183
}
