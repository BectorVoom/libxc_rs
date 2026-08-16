//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta303 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1189;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1190;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1191;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta303<F: Float>(t3475: F, t431: F, t426: F, t1168: F, t3453: F, t3479: F, t12252: F, t12259: F, t12261: F, t12263: F, t12265: F, t12271: F, t12275: F, t12279: F, t12284: F, t12289: F, t12292: F, t12323: F, t12329: F, t12332: F, t12295: F, t12351: F, t12297: F, t12299: F, t12301: F, t12303: F, t12307: F, t12310: F, t12314: F, t12317: F, t12320: F, t12344: F, t12347: F, t12354: F, t1169: F, t1159: F, t3478: F, t434: F, t1179: F, t3488: F, t1175: F, t3520: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t12428, t12429, t12430, t12431, t12448) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1189::<F>(t3475, t431, t426, t1168, t3453, t3479, t12252, t12259, t12261, t12263, t12265, t12271, t12275, t12279, t12284, t12289, t12292, t12323, t12329, t12332);
        let t12463 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1190::<F>(t12295, t12351, t12297, t12299, t12301, t12303, t12307, t12310, t12314, t12317, t12320, t12344, t12347, t12354);
        let (t12464, t12465, t12469, t12470, t12472, t12473, t12476, t12481) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1191::<F>(t12448, t12463, t1169, t1159, t3475, t426, t3478, t434, t12430, t1179, t3488, t1175, t3520);
    (t12428, t12429, t12430, t12431, t12464, t12465, t12469, t12470, t12472, t12473, t12476, t12481)
}
