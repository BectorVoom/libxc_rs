//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta523 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2040;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2041;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2042;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta523<F: Float>(t21342: F, t225: F, t494: F, t1294: F, t6702: F, t13182: F, t1210: F, t12628: F, t1274: F, t1295: F, t1775: F, t17973: F, t17995: F, t18005: F, t18065: F, t18097: F, t1829: F, t20741: F, t20744: F, t20748: F, t20753: F, t20756: F, t20760: F, t3572: F, t460: F, t5220: F, t5225: F, t5231: F, t5246: F, t5498: F, t6588: F, t1828: F, t5245: F, t1277: F, t1774: F, t5497: F, t3736: F, t5428: F, t1204: F, t1770: F, t17986: F, t18054: F, t18062: F, t18087: F, t18114: F, t3556: F, t3561: F, t5251: F, t5414: F, t5423: F, t6580: F, t6697: F, t6703: F, t1811: F, t5219: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t21344, t21347, t21348, t21357) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2040::<F>(t21342, t225, t494, t1294, t6702, t13182, t1210, t12628, t1274, t1295, t1775, t17973, t17995, t18005, t18065, t18097, t1829, t20741, t20744, t20748, t20753, t20756, t20760, t3572, t460, t5220, t5225, t5231, t5246, t5498, t6588);
        let (t21365, t21366, t21382, t21389, t21390, t21393) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2041::<F>(t1828, t5245, t1277, t1774, t5497, t3736, t5428, t1204, t1210, t1770, t1775, t17986, t18054, t18062, t18087, t18114, t1829, t3556, t3561, t5220, t5246, t5251, t5414, t5423, t6580, t6588, t6697, t6703);
        let t21394 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2042::<F>(t1811, t5219);
    (t21344, t21347, t21348, t21357, t21365, t21366, t21382, t21389, t21390, t21393, t21394)
}
