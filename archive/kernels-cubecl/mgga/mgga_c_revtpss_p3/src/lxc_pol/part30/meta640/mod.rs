//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta640 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2224;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2225;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2226;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2227;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta640<F: Float>(t355: F, t5352: F, t17288: F, t2142: F, t5216: F, t11239: F, t1811: F, t1276: F, t2148: F, t1209: F, t2143: F, t1203: F, t1215: F, t1295: F, t1770: F, t17988: F, t18019: F, t26886: F, t26909: F, t26994: F, t27008: F, t27011: F, t27020: F, t29212: F, t29236: F, t29278: F, t29296: F, t5215: F, t5237: F, t5245: F, t5497: F, t5498: F, t7602: F, t7627: F, t7636: F, t7637: F, t7643: F, t7651: F, t7652: F, t96927: F, t96953: F, t96954: F, t97041: F, t104490: F, t1204: F, t17331: F, t2144: F, t26918: F, t26922: F, t26937: F, t26969: F, t26999: F, t29118: F, t29124: F, t29149: F, t29166: F, t29183: F, t29227: F, t3551: F, t3568: F, t3738: F, t3791: F, t5231: F, t5423: F, t7629: F, t8190: F, t8197: F, t8198: F, t8201: F, t97019: F, t97078: F, t97475: F, t3601: F, t26852: F, t5378: F, t29083: F, t3636: F, t1234: F, t29082: F, t17620: F, t26870: F, t26865: F, t370: F, t17727: F, t17423: F, t29097: F, t17789: F, t29100: F, t17416: F, t7624: F, t17214: F, t17484: F, t17580: F, t17760: F, t29037: F, t29040: F, t3620: F, t3640: F, t3644: F, t97149: F, t97261: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t104510, t104529, t104560) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2224::<F>(t355, t5352, t17288, t2142, t5216, t11239, t1811, t1276, t2148, t1209, t2143, t1203, t1215, t1295, t1770, t17988, t18019, t26886, t26909, t26994, t27008, t27011, t27020, t29212, t29236, t29278, t29296, t5215, t5237, t5245, t5497, t5498, t7602, t7627, t7636, t7637, t7643, t7651, t7652, t96927, t96953, t96954, t97041);
        let t104601 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2225::<F>(t104490, t1203, t1204, t17331, t2144, t26918, t26922, t26937, t26969, t26994, t26999, t27011, t29118, t29124, t29149, t29166, t29183, t29227, t3551, t3568, t3738, t3791, t5216, t5231, t5423, t7629, t7637, t7651, t8190, t8197, t8198, t8201, t97019, t97078, t97475);
        let (t104606, t104624, t104626, t104636, t104640, t104646) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2226::<F>(t3601, t8201, t26852, t5378, t29083, t3636, t1234, t29082, t17620, t26870, t26865, t370);
        let (t104647, t104666) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2227::<F>(t104646, t17727, t17423, t29097, t17789, t29100, t17416, t7624, t17214, t17484, t17580, t17760, t29037, t29040, t29083, t3620, t3640, t3644, t97149, t97261);
    (t104510, t104529, t104560, t104601, t104606, t104624, t104626, t104636, t104640, t104646, t104647, t104666)
}
