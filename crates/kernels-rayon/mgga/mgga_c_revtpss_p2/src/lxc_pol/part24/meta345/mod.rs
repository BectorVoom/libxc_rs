//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta345 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1198;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1199;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1200;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1201;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1202;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta345(t11534: f64, t15189: f64, t18919: f64, t18924: f64, t18934: f64, t23479: f64, t23483: f64, t23487: f64, t23490: f64, t23501: f64, t23505: f64, t291: f64, t15123: f64, t23472: f64, t23476: f64, t23493: f64, t23496: f64, t23508: f64, t23511: f64, t11479: f64, t11480: f64, t19002: f64, t19004: f64, t19009: f64, t23521: f64, t23523: f64, t23536: f64, t23538: f64, t23541: f64, t23543: f64, t964: f64, t973: f64, t981: f64, t1621: f64, t6157: f64, t954: f64, t23451: f64, t11509: f64, t11507: f64, t15104: f64, t15413: f64, t1622: f64, t19173: f64, t23461: f64, t23463: f64, t23465: f64, t23469: f64, t23549: f64, t23552: f64, t23564: f64, t23567: f64, t2968: f64, t3012: f64, t4647: f64, t6158: f64, t6174: f64, t6190: f64, t965: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23663, t23665) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1198(t11534, t15189, t18919, t18924, t18934, t23479, t23483, t23487, t23490, t23501, t23505, t291);
        let t23680 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1199(t15123, t15189, t23472, t23476, t23479, t23483, t23487, t23490, t23493, t23496, t23501, t23505, t23508, t23511);
        let t23693 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1200(t11479, t11480, t18919, t18924, t18934, t19002, t19004, t19009, t23521, t23523, t23536, t23538, t23541, t23543);
        let t23694 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1201(t23680, t23693);
        let (t23696, t23698, t23705, t23706, t23711, t23714, t23717, t23720) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1202(t23694, t964, t973, t981, t1621, t6157, t954, t23451, t11509, t11507, t15104, t15413, t1622, t19173, t23461, t23463, t23465, t23469, t23549, t23552, t23564, t23567, t2968, t3012, t4647, t6158, t6174, t6190, t965);
    (t23663, t23665, t23694, t23696, t23698, t23705, t23706, t23711, t23714, t23717, t23720)
}
