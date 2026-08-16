//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2439/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2439(t13969: f64, t14102: f64, t3039: f64, t10876: f64, t13990: f64, t14134: f64, t3048: f64, t1025: f64, t10957: f64, t14093: f64, t42735: f64, t42752: f64, t43094: f64, t43097: f64, t4636: f64, t49866: f64, t49872: f64, t49873: f64, t49877: f64) -> f64 {
    let t49884 = t3039 * t13969 * t14102;
    let t49887 = t10876 * t13969 * t13990;
    let t49889 = t3048 * t14134;
    let t49891 = t42735 / 4608.0_f64 + t42752 / 5184.0_f64 + t49866 * t1025 / 1024.0_f64 - t49872 - t49873 / 576.0_f64 + 19.0_f64 / 864.0_f64 * t10957 * t4636 - t49877 / 216.0_f64 - t3048 * t14093 / 288.0_f64 + t43094 / 768.0_f64 - t43097 / 1536.0_f64 - t49884 / 1536.0_f64 - t49887 / 256.0_f64 + t49889 / 108.0_f64;
    t49891
}
