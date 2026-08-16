//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta323 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1234;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1235;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta323<F: Float>(t12948: F, t3610: F, t1263: F, t3584: F, t1122: F, t1042: F, t1260: F, t3666: F, t3172: F, t3713: F, t3711: F, t127: F, t3661: F, t371: F, t1235: F, t12640: F, t225: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t12949, t12951, t12952, t12953, t12956) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1234::<F>(t12948, t3610, t1263, t3584, t1122, t1042, t1260, t3666);
        let (t12959, t12960, t12963, t12964, t12966) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1235::<F>(t3172, t3713, t3711, t127, t3661, t371, t1235, t12640, t225);
    (t12949, t12951, t12952, t12953, t12956, t12959, t12960, t12963, t12964, t12966)
}
