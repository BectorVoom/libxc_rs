//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta562 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2257;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2258;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta562(t17376: f64, t3599: f64, t17482: f64, t3604: f64, t3720: f64, t3372: f64, t5277: f64, t1042: f64, t12855: f64, t12964: f64, t12979: f64, t12985: f64, t12996: f64, t17569: f64, t3606: f64, t3620: f64, t3640: f64, t3711: f64, t3714: f64, t5381: f64, t5391: f64, t3368: f64, t3704: f64, t5274: f64, t1774: f64, t3588: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17572, t17579, t17580, t17583, t17584, t17587) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2257(t17376, t3599, t17482, t3604, t3720, t3372, t5277, t1042, t12855, t12964, t12979, t12985, t12996, t17569, t3606, t3620, t3640, t3711, t3714, t5381, t5391);
        let (t17588, t17589, t17593, t17600) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2258(t3368, t5277, t1042, t3704, t5274, t1774, t3588);
    (t17572, t17579, t17580, t17583, t17584, t17587, t17588, t17589, t17593, t17600)
}
