//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta597 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2482;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2483;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta597(t19077: f64, t291: f64, t4719: f64, t4734: f64, t6226: f64, t974: f64, t981: f64, t15170: f64, t15189: f64, t15447: f64, t15457: f64, t15459: f64, t18944: f64, t18961: f64, t18964: f64, t18967: f64, t18970: f64, t18973: f64, t11134: f64, t11334: f64, t11338: f64, t11366: f64, t18948: f64, t19002: f64, t19004: f64, t19007: f64, t19009: f64, t19014: f64, t19017: f64, t15123: f64, t15127: f64, t15435: f64, t18906: f64, t18911: f64, t18915: f64, t18919: f64, t18924: f64, t18928: f64, t18932: f64, t18934: f64, t18939: f64, t18951: f64, t18980: f64, t18982: f64, t18985: f64, t18988: f64, t18990: f64, t18993: f64, t18995: f64) -> (f64, f64, f64, f64, f64) {
        let (t19079, t19081, t19082, t19084, t19103) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2482(t19077, t291, t4719, t4734, t6226, t974, t981, t15170, t15189, t15447, t15457, t15459, t18944, t18961, t18964, t18967, t18970, t18973);
        let t19127 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2483(t11134, t11334, t11338, t11366, t18948, t19002, t19004, t19007, t19009, t19014, t19017, t15123, t15127, t15435, t18906, t18911, t18915, t18919, t18924, t18928, t18932, t18934, t18939, t18951, t18980, t18982, t18985, t18988, t18990, t18993, t18995, t19103);
    (t19079, t19081, t19082, t19084, t19127)
}
