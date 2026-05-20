//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta582 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2292;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2293;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta582<F: Float>(t1269: F, t1770: F, t1214: F, t5497: F, t1277: F, t1211: F, t17345: F, t1811: F, t3555: F, t1210: F, t1215: F, t12628: F, t12633: F, t12641: F, t12658: F, t1295: F, t13177: F, t17331: F, t1775: F, t3561: F, t3572: F, t3576: F, t3732: F, t3739: F, t495: F, t5231: F, t5251: F, t5417: F, t5423: F, t5429: F, t5498: F, t1294: F, t5245: F, t1774: F, t3737: F, t3738: F, t460: F, t5412: F, t17306: F, t487: F, t5219: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t18005, t18019, t18030, t18037, t18040) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2292::<F>(t1269, t1770, t1214, t5497, t1277, t1211, t17345, t1811, t3555, t1210, t1215, t12628, t12633, t12641, t12658, t1295, t13177, t17331, t1775, t3561, t3572, t3576, t3732, t3739, t495, t5231, t5251, t5417, t5423, t5429, t5498);
        let (t18043, t18047, t18054, t18059, t18062) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2293::<F>(t1294, t5245, t1277, t1774, t3737, t3738, t460, t5412, t17306, t487, t1269, t5219);
    (t18005, t18019, t18030, t18037, t18040, t18043, t18047, t18054, t18059, t18062)
}
