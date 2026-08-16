//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta484 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1649;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1650;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1651;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1652;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta484<F: Float>(t26163: F, t26558: F, t193: F, t200: F, t2056: F, t7841: F, t865: F, t2718: F, t25049: F, t4234: F, t7101: F, t1510: F, t24269: F, t1499: F, t2051: F, t23003: F, t23026: F, t23029: F, t23167: F, t23170: F, t24246: F, t24250: F, t24265: F, t25239: F, t25243: F, t25246: F, t25252: F, t25259: F, t2617: F, t4162: F, t4166: F, t7102: F, t7104: F, t7837: F, t812: F, t25277: F, t25077: F, t25080: F, t23114: F, t23120: F, t24218: F, t24220: F, t24221: F, t25085: F, t25087: F, t25089: F, t25091: F, t25095: F, t25099: F, t25140: F, t25144: F, t23125: F, t23135: F, t24230: F, t24231: F, t25142: F, t25147: F, t25149: F, t25151: F, t25156: F, t23043: F, t23063: F, t23071: F, t23084: F, t25065: F, t25069: F, t25071: F, t25073: F, t25107: F, t25109: F, t25113: F, t25117: F, t25121: F, t25124: F, t25126: F, t25128: F, t25133: F, t25136: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t26559, t26563) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1649::<F>(t26163, t26558, t193, t200, t2056);
        let (t26582, t26591, t26598, t26608, t26611) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1650::<F>(t7841, t865, t2718, t25049, t4234, t7101, t1510, t24269, t1499, t2051, t23003, t23026, t23029, t23167, t23170, t24246, t24250, t24265, t25239, t25243, t25246, t25252, t25259, t2617, t4162, t4166, t7102, t7104, t7837, t812);
        let (t26613, t26619, t26621, t26630) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1651::<F>(t25277, t25077, t25080, t23114, t23120, t24218, t24220, t24221, t25085, t25087, t25089, t25091, t25095, t25099);
        let t26653 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1652::<F>(t25140, t25144, t23125, t23135, t24230, t24231, t25142, t25147, t25149, t25151, t25156, t23043, t23063, t23071, t23084, t25065, t25069, t25071, t25073, t25107, t25109, t25113, t25117, t25121, t25124, t25126, t25128, t25133, t25136, t26619, t26621, t26630);
    (t26559, t26563, t26582, t26591, t26598, t26608, t26611, t26613, t26653)
}
