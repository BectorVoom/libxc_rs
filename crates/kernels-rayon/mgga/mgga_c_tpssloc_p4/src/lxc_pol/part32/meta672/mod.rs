//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta672 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2105;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2106;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta672(t15643: f64, t7345: f64, t27639: f64, t86264: f64, t27645: f64, t3540: f64, t8043: f64, t2136: f64, t607: f64, t8027: f64, t1714: f64, t24682: f64, t460: f64, t27628: f64, t27634: f64, t10469: f64, t24719: f64, t3: f64, t86154: f64, t2132: f64, t24746: f64, t1222: f64, t27589: f64, t1184: f64, t1409: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t95352, t95362, t95364, t95365, t95370, t95382, t95384) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2105(t15643, t7345, t27639, t86264, t27645, t3540, t8043, t2136, t607, t8027, t1714, t24682, t460);
        let (t95387, t95396, t95404, t95410, t95413) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2106(t27628, t27634, t10469, t24719, t3, t86154, t2132, t24746, t95382, t1222, t27589, t1184, t1409);
    (t95352, t95362, t95364, t95365, t95370, t95384, t95387, t95396, t95404, t95410, t95413)
}
