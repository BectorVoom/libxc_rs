//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta640 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2224;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2225;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2226;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2227;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta640(t355: f64, t5352: f64, t17288: f64, t2142: f64, t5216: f64, t11239: f64, t1811: f64, t1276: f64, t2148: f64, t1209: f64, t2143: f64, t1203: f64, t1215: f64, t1295: f64, t1770: f64, t17988: f64, t18019: f64, t26886: f64, t26909: f64, t26994: f64, t27008: f64, t27011: f64, t27020: f64, t29212: f64, t29236: f64, t29278: f64, t29296: f64, t5215: f64, t5237: f64, t5245: f64, t5497: f64, t5498: f64, t7602: f64, t7627: f64, t7636: f64, t7637: f64, t7643: f64, t7651: f64, t7652: f64, t96927: f64, t96953: f64, t96954: f64, t97041: f64, t104490: f64, t1204: f64, t17331: f64, t2144: f64, t26918: f64, t26922: f64, t26937: f64, t26969: f64, t26999: f64, t29118: f64, t29124: f64, t29149: f64, t29166: f64, t29183: f64, t29227: f64, t3551: f64, t3568: f64, t3738: f64, t3791: f64, t5231: f64, t5423: f64, t7629: f64, t8190: f64, t8197: f64, t8198: f64, t8201: f64, t97019: f64, t97078: f64, t97475: f64, t3601: f64, t26852: f64, t5378: f64, t29083: f64, t3636: f64, t1234: f64, t29082: f64, t17620: f64, t26870: f64, t26865: f64, t370: f64, t17727: f64, t17423: f64, t29097: f64, t17789: f64, t29100: f64, t17416: f64, t7624: f64, t17214: f64, t17484: f64, t17580: f64, t17760: f64, t29037: f64, t29040: f64, t3620: f64, t3640: f64, t3644: f64, t97149: f64, t97261: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t104510, t104529, t104560) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2224(t355, t5352, t17288, t2142, t5216, t11239, t1811, t1276, t2148, t1209, t2143, t1203, t1215, t1295, t1770, t17988, t18019, t26886, t26909, t26994, t27008, t27011, t27020, t29212, t29236, t29278, t29296, t5215, t5237, t5245, t5497, t5498, t7602, t7627, t7636, t7637, t7643, t7651, t7652, t96927, t96953, t96954, t97041);
        let t104601 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2225(t104490, t1203, t1204, t17331, t2144, t26918, t26922, t26937, t26969, t26994, t26999, t27011, t29118, t29124, t29149, t29166, t29183, t29227, t3551, t3568, t3738, t3791, t5216, t5231, t5423, t7629, t7637, t7651, t8190, t8197, t8198, t8201, t97019, t97078, t97475);
        let (t104606, t104624, t104626, t104636, t104640, t104646) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2226(t3601, t8201, t26852, t5378, t29083, t3636, t1234, t29082, t17620, t26870, t26865, t370);
        let (t104647, t104666) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2227(t104646, t17727, t17423, t29097, t17789, t29100, t17416, t7624, t17214, t17484, t17580, t17760, t29037, t29040, t29083, t3620, t3640, t3644, t97149, t97261);
    (t104510, t104529, t104560, t104601, t104606, t104624, t104626, t104636, t104640, t104646, t104647, t104666)
}
