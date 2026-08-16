//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1174/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1174(t1167: f64, t14149: f64, t944: f64, t3324: f64, t4063: f64, t14605: f64, t14611: f64, t14655: f64, t14689: f64, t14708: f64, t14716: f64, t14745: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14829 = t14149 * t1167;
    let t14831 = t1167 * t944;
    let t14835 = t4063 * t3324;
    let t14898 = 7.0_f64 / 2304.0_f64 * t14605;
    let t14931 = 7.0_f64 / 2304.0_f64 * t14611;
    let t14962 = 7.0_f64 / 576.0_f64 * t14655;
    let t14974 = 7.0_f64 / 144.0_f64 * t14689;
    let t14978 = 7.0_f64 / 144.0_f64 * t14708;
    let t14986 = 7.0_f64 / 1152.0_f64 * t14716;
    let t14996 = 7.0_f64 / 72.0_f64 * t14745;
    (t14829, t14831, t14835, t14898, t14931, t14962, t14974, t14978, t14986, t14996)
}
