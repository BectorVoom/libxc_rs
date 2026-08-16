//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta589 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2220;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2221;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2222;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2223;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2224;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta589(t23640: f64, t373: f64, t11257: f64, t1042: f64, t11506: f64, t23451: f64, t11509: f64, t981: f64, t23448: f64, t23450: f64, t23461: f64, t23463: f64, t23465: f64, t23469: f64, t23549: f64, t23552: f64, t23554: f64, t23556: f64, t11534: f64, t15189: f64, t18919: f64, t18924: f64, t18934: f64, t23479: f64, t23483: f64, t23487: f64, t23490: f64, t23501: f64, t23505: f64, t291: f64, t15123: f64, t23472: f64, t23476: f64, t23493: f64, t23496: f64, t23508: f64, t23511: f64, t11479: f64, t11480: f64, t19002: f64, t19004: f64, t19009: f64, t23521: f64, t23523: f64, t23536: f64, t23538: f64, t23541: f64, t23543: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23641, t23642, t23643, t23648, t23649, t23651, t23652) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2220(t23640, t373, t11257, t1042, t11506, t23451, t11509, t981, t23448, t23450, t23461, t23463, t23465, t23469, t23549, t23552, t23554, t23556);
        let (t23663, t23665) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2221(t11534, t15189, t18919, t18924, t18934, t23479, t23483, t23487, t23490, t23501, t23505, t291);
        let t23680 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2222(t15123, t15189, t23472, t23476, t23479, t23483, t23487, t23490, t23493, t23496, t23501, t23505, t23508, t23511);
        let t23693 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2223(t11479, t11480, t18919, t18924, t18934, t19002, t19004, t19009, t23521, t23523, t23536, t23538, t23541, t23543);
        let t23694 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2224(t23680, t23693);
    (t23641, t23642, t23643, t23648, t23649, t23651, t23652, t23663, t23665, t23694)
}
