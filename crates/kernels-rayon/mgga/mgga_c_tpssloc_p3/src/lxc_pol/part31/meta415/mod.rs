//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta415 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1518;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1519;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1520;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta415(t1352: f64, t19956: f64, t5248: f64, t5250: f64, t5249: f64, t5287: f64, t19871: f64, t120: f64, t6330: f64, t12419: f64, t6347: f64, t3805: f64, t5187: f64, t550: f64, t1307: f64, t3870: f64, t820: f64, t19744: f64, t12369: f64, t12346: f64, t12366: f64, t12429: f64, t1363: f64, t16233: f64, t16394: f64, t16400: f64, t19940: f64, t19942: f64, t19945: f64, t19951: f64, t19958: f64, t3803: f64, t5246: f64, t5259: f64, t6396: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19962, t19966, t19972, t19976, t19981, t19986) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1518(t1352, t19956, t5248, t5250, t5249, t5287, t19871, t120, t6330, t12419, t6347, t3805);
        let (t19991, t19994, t19996, t20000, t20004) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1519(t5187, t550, t3805, t5249, t1307, t6347, t3870, t820, t19744, t19871, t5248, t12369);
        let t20007 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1520(t12346, t12366, t12429, t1363, t16233, t16394, t16400, t19940, t19942, t19945, t19951, t19958, t19962, t19966, t19972, t19976, t19981, t19986, t19991, t19996, t20000, t20004, t3803, t5246, t5259, t6396);
    (t19962, t19966, t19972, t19976, t19981, t19986, t19991, t19994, t19996, t20000, t20004, t20007)
}
