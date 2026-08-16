//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 851/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk851(t34777: f64, t901: f64, t35106: f64, t40219: f64, t40225: f64, t41915: f64, t41919: f64, t41922: f64, t41927: f64, t41930: f64, t41933: f64, t41935: f64, t41938: f64, t41941: f64, t41942: f64, t41945: f64, t41948: f64, t41950: f64, t41952: f64, t41954: f64, t41958: f64) -> f64 {
    let t41960 = t34777 * t901;
    let t41962 = t35106 * t901;
    let t41964 = t41915 + t41919 + t41922 + 0.76685851907841499352e0_f64 * t40219 - t41927 - t41930 + t41933 + 0.30674340763136599741e2_f64 * t41935 + 0.95334639871601137787e0_f64 * t41938 + t41941 + 0.71500979903700853338e0_f64 * t41942 + t41945 - 0.1533717038156829987e1_f64 * t40225 - t41948 - t41950 - t41952 + 0.51123901271894332901e0_f64 * t41954 - 0.89376224879626066674e-1_f64 * t41958 + 0.29792074959875355558e-1_f64 * t41960 + 0.29792074959875355558e-1_f64 * t41962;
    t41964
}
