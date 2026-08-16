//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta456 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1890;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1891;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta456<F: Float>(t19572: F, t4983: F, t4998: F, t19482: F, t999: F, t19501: F, t1089: F, t1678: F, t4866: F, t3153: F, t6271: F, t3298: F, t342: F, t1024: F, t1087: F, t1090: F, t12116: F, t12122: F, t12127: F, t16381: F, t1647: F, t1689: F, t1692: F, t19557: F, t19566: F, t19569: F, t3278: F, t4743: F, t4857: F, t4954: F, t4970: F, t4981: F, t4984: F, t4996: F, t4999: F, t5009: F, t5012: F, t6375: F, t6383: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t19573, t19576, t19579, t19580, t19584, t19593, t19594, t19597, t19602) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1890::<F>(t19572, t4983, t4998, t19482, t999, t19501, t1089, t1678, t4866, t3153, t6271, t3298);
        let (t19603, t19606) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1891::<F>(t19602, t342, t1024, t1087, t1090, t12116, t12122, t12127, t16381, t1647, t1689, t1692, t19557, t19566, t19569, t19573, t19576, t19580, t19584, t19594, t19597, t3278, t4743, t4857, t4954, t4970, t4981, t4984, t4996, t4999, t5009, t5012, t6375, t6383);
    (t19573, t19576, t19579, t19580, t19584, t19593, t19594, t19597, t19602, t19603, t19606)
}
