//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta611 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2514;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2515;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta611(t19572: f64, t4983: f64, t4998: f64, t19482: f64, t999: f64, t19501: f64, t1089: f64, t1678: f64, t4866: f64, t3153: f64, t6271: f64, t3298: f64, t342: f64, t1024: f64, t1087: f64, t1090: f64, t12116: f64, t12122: f64, t12127: f64, t16381: f64, t1647: f64, t1689: f64, t1692: f64, t19557: f64, t19566: f64, t19569: f64, t3278: f64, t4743: f64, t4857: f64, t4954: f64, t4970: f64, t4981: f64, t4984: f64, t4996: f64, t4999: f64, t5009: f64, t5012: f64, t6375: f64, t6383: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19573, t19576, t19580, t19584, t19593, t19594, t19597, t19602) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2514(t19572, t4983, t4998, t19482, t999, t19501, t1089, t1678, t4866, t3153, t6271, t3298);
        let (t19603, t19606) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2515(t19602, t342, t1024, t1087, t1090, t12116, t12122, t12127, t16381, t1647, t1689, t1692, t19557, t19566, t19569, t19573, t19576, t19580, t19584, t19594, t19597, t3278, t4743, t4857, t4954, t4970, t4981, t4984, t4996, t4999, t5009, t5012, t6375, t6383);
    (t19573, t19576, t19580, t19584, t19593, t19594, t19597, t19602, t19603, t19606)
}
