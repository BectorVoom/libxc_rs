//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta641 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2109;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2110;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2111;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2112;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta641(t4191: f64, t81865: f64, t13302: f64, t23146: f64, t13322: f64, t4250: f64, t13316: f64, t13312: f64, t81749: f64, t23145: f64, t4166: f64, t2649: f64, t22690: f64, t234: f64, t7496: f64, t776: f64, t81792: f64, t23109: f64, t23110: f64, t232: f64, t236: f64, t4233: f64, t25132: f64, t81876: f64, t13336: f64, t1898: f64, t249: f64, t23047: f64, t2635: f64, t81736: f64, t81743: f64, t81750: f64, t87183: f64, t1516: f64, t81766: f64, t23127: f64, t4261: f64, t13347: f64, t6621: f64, t131: f64, t6598: f64, t9537: f64, t225: f64, t2627: f64, t25093: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t87185, t87187, t87189, t87191, t87193, t87195, t87198, t87200) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2109(t4191, t81865, t13302, t23146, t13322, t4250, t13316, t13312, t81749, t23145, t4166, t2649);
        let (t87202, t87206, t87212, t87213) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2110(t22690, t234, t7496, t776, t81792, t23109, t23110, t232, t236, t4233, t25132, t81876);
        let t87221 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2111(t13336, t1898, t249, t23047, t4166, t2635, t81736, t81743, t81750, t87183, t87185, t87187, t87189, t87191, t87193, t87195, t87198, t87200, t87206, t87212, t87213);
        let (t87222, t87224, t87226, t87229, t87230, t87233) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2112(t1516, t81766, t23127, t4261, t13347, t6621, t131, t6598, t9537, t225, t2627, t236, t25093);
    (t87202, t87221, t87222, t87224, t87226, t87229, t87230, t87233)
}
