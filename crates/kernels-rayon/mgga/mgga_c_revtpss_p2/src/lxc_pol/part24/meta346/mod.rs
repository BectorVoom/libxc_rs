//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta346 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1203;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1204;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1205;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta346(t23705: f64, t2970: f64, t15123: f64, t15189: f64, t23472: f64, t23476: f64, t23479: f64, t23483: f64, t23487: f64, t23490: f64, t23493: f64, t23496: f64, t23501: f64, t23505: f64, t23508: f64, t23511: f64, t11422: f64, t11423: f64, t18919: f64, t18924: f64, t18934: f64, t19002: f64, t19004: f64, t19009: f64, t23521: f64, t23523: f64, t23536: f64, t23538: f64, t23541: f64, t23543: f64, t954: f64, t1621: f64, t19275: f64, t1634: f64, t6205: f64, t1633: f64, t19303: f64, t1610: f64, t6141: f64, t2874: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23723, t23740) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1203(t23705, t2970, t15123, t15189, t23472, t23476, t23479, t23483, t23487, t23490, t23493, t23496, t23501, t23505, t23508, t23511);
        let t23753 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1204(t11422, t11423, t18919, t18924, t18934, t19002, t19004, t19009, t23521, t23523, t23536, t23538, t23541, t23543);
        let (t23754, t23755, t23758, t23761, t23764, t23767, t23769) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1205(t23740, t23753, t954, t1621, t19275, t1634, t6205, t1633, t19303, t1610, t6141, t2874);
    (t23723, t23754, t23755, t23758, t23761, t23764, t23767, t23769)
}
