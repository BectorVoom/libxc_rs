//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta356 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1672;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1673;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1674;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta356(t12345: f64, t1369: f64, t12215: f64, t12317: f64, t12320: f64, t12323: f64, t12325: f64, t12330: f64, t12331: f64, t12335: f64, t12336: f64, t12340: f64, t3783: f64, t3876: f64, t559: f64, t241: f64, t67: f64, t6924: f64, t12156: f64, t820: f64, t3866: f64, t3872: f64, t12012: f64, t1367: f64, t1339: f64, t2690: f64, t1336: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12346, t12348) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1672(t12345, t1369, t12215, t12317, t12320, t12323, t12325, t12330, t12331, t12335, t12336, t12340, t3783, t3876, t559);
        let (t12351, t12353, t12356, t12358, t12361, t12364) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1673(t241, t67, t6924, t12156, t820, t3866, t3872, t3876, t12012, t1367, t1339, t2690);
        let t12365 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1674(t12364, t1336);
    (t12346, t12348, t12351, t12353, t12356, t12358, t12361, t12364, t12365)
}
