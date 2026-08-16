//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta484 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1649;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1650;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1651;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1652;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta484(t26163: f64, t26558: f64, t193: f64, t200: f64, t2056: f64, t7841: f64, t865: f64, t2718: f64, t25049: f64, t4234: f64, t7101: f64, t1510: f64, t24269: f64, t1499: f64, t2051: f64, t23003: f64, t23026: f64, t23029: f64, t23167: f64, t23170: f64, t24246: f64, t24250: f64, t24265: f64, t25239: f64, t25243: f64, t25246: f64, t25252: f64, t25259: f64, t2617: f64, t4162: f64, t4166: f64, t7102: f64, t7104: f64, t7837: f64, t812: f64, t25277: f64, t25077: f64, t25080: f64, t23114: f64, t23120: f64, t24218: f64, t24220: f64, t24221: f64, t25085: f64, t25087: f64, t25089: f64, t25091: f64, t25095: f64, t25099: f64, t25140: f64, t25144: f64, t23125: f64, t23135: f64, t24230: f64, t24231: f64, t25142: f64, t25147: f64, t25149: f64, t25151: f64, t25156: f64, t23043: f64, t23063: f64, t23071: f64, t23084: f64, t25065: f64, t25069: f64, t25071: f64, t25073: f64, t25107: f64, t25109: f64, t25113: f64, t25117: f64, t25121: f64, t25124: f64, t25126: f64, t25128: f64, t25133: f64, t25136: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26559, t26563) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1649(t26163, t26558, t193, t200, t2056);
        let (t26582, t26591, t26598, t26608, t26611) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1650(t7841, t865, t2718, t25049, t4234, t7101, t1510, t24269, t1499, t2051, t23003, t23026, t23029, t23167, t23170, t24246, t24250, t24265, t25239, t25243, t25246, t25252, t25259, t2617, t4162, t4166, t7102, t7104, t7837, t812);
        let (t26613, t26619, t26621, t26630) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1651(t25277, t25077, t25080, t23114, t23120, t24218, t24220, t24221, t25085, t25087, t25089, t25091, t25095, t25099);
        let t26653 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1652(t25140, t25144, t23125, t23135, t24230, t24231, t25142, t25147, t25149, t25151, t25156, t23043, t23063, t23071, t23084, t25065, t25069, t25071, t25073, t25107, t25109, t25113, t25117, t25121, t25124, t25126, t25128, t25133, t25136, t26619, t26621, t26630);
    (t26559, t26563, t26582, t26591, t26598, t26608, t26611, t26613, t26653)
}
