//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta303 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1189;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1190;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1191;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta303(t3475: f64, t431: f64, t426: f64, t1168: f64, t3453: f64, t3479: f64, t12252: f64, t12259: f64, t12261: f64, t12263: f64, t12265: f64, t12271: f64, t12275: f64, t12279: f64, t12284: f64, t12289: f64, t12292: f64, t12323: f64, t12329: f64, t12332: f64, t12295: f64, t12351: f64, t12297: f64, t12299: f64, t12301: f64, t12303: f64, t12307: f64, t12310: f64, t12314: f64, t12317: f64, t12320: f64, t12344: f64, t12347: f64, t12354: f64, t1169: f64, t1159: f64, t3478: f64, t434: f64, t1179: f64, t3488: f64, t1175: f64, t3520: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12428, t12429, t12430, t12431, t12448) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1189(t3475, t431, t426, t1168, t3453, t3479, t12252, t12259, t12261, t12263, t12265, t12271, t12275, t12279, t12284, t12289, t12292, t12323, t12329, t12332);
        let t12463 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1190(t12295, t12351, t12297, t12299, t12301, t12303, t12307, t12310, t12314, t12317, t12320, t12344, t12347, t12354);
        let (t12464, t12465, t12469, t12470, t12472, t12473, t12476, t12481) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1191(t12448, t12463, t1169, t1159, t3475, t426, t3478, t434, t12430, t1179, t3488, t1175, t3520);
    (t12428, t12429, t12430, t12431, t12464, t12465, t12469, t12470, t12472, t12473, t12476, t12481)
}
