//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta562 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2396;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2397;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2398;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta562(t1269: f64, t1770: f64, t1214: f64, t5497: f64, t1277: f64, t1211: f64, t17345: f64, t1811: f64, t3555: f64, t1210: f64, t1215: f64, t12628: f64, t12633: f64, t12641: f64, t12658: f64, t1295: f64, t13177: f64, t17331: f64, t1775: f64, t3561: f64, t3572: f64, t3576: f64, t3732: f64, t3739: f64, t495: f64, t5231: f64, t5251: f64, t5417: f64, t5423: f64, t5429: f64, t5498: f64, t1294: f64, t5245: f64, t1774: f64, t3737: f64, t3738: f64, t460: f64, t5412: f64, t17306: f64, t487: f64, t5219: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18005, t18018, t18019, t18030, t18037) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2396(t1269, t1770, t1214, t5497, t1277, t1211, t17345, t1811, t3555);
        let t18040 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2397(t1210, t1215, t12628, t12633, t12641, t12658, t1295, t13177, t17331, t1775, t18005, t18019, t18030, t18037, t3561, t3572, t3576, t3732, t3739, t495, t5231, t5251, t5417, t5423, t5429, t5498);
        let (t18042, t18043, t18047, t18054, t18059, t18062) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2398(t1294, t5245, t1277, t1774, t3737, t3738, t460, t5412, t17306, t487, t1269, t5219);
    (t18005, t18018, t18019, t18030, t18037, t18040, t18042, t18043, t18047, t18054, t18059, t18062)
}
