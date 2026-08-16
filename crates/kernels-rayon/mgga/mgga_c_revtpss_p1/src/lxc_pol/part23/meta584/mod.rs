//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta584 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2211;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2212;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2213;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta584(t4578: f64, t5825: f64, t904: f64, t128: f64, t23499: f64, t2908: f64, t141: f64, t930: f64, t15123: f64, t15189: f64, t23472: f64, t23476: f64, t23479: f64, t23483: f64, t23487: f64, t23490: f64, t23493: f64, t23496: f64, t23501: f64, t4598: f64, t6120: f64, t4614: f64, t11304: f64, t18919: f64, t18924: f64, t18934: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23503, t23504, t23505) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2211(t4578, t5825, t904, t128);
        let (t23507, t23508, t23510, t23511, t23514) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2212(t23499, t2908, t141, t23503, t930, t15123, t15189, t23472, t23476, t23479, t23483, t23487, t23490, t23493, t23496, t23501, t23505);
        let (t23521, t23523, t23535) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2213(t4598, t6120, t4614, t11304, t15189, t18919, t18924, t18934, t23479, t23483, t23487, t23490, t23501, t23505);
    (t23503, t23504, t23505, t23507, t23508, t23510, t23511, t23514, t23521, t23523, t23535)
}
