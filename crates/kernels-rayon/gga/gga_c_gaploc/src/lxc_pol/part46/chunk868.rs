//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 868/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk868(t40395: f64, t41839: f64, t6710: f64, t6711: f64, t204: f64, t41878: f64, t587: f64, t2890: f64, t9267: f64, t9278: f64, t40374: f64, t40380: f64, t40397: f64, t40400: f64, t42144: f64, t42146: f64, t42151: f64, t42154: f64, t42157: f64, t42159: f64, t42161: f64, t42163: f64, t42166: f64, t42168: f64, t42170: f64, t42172: f64) -> f64 {
    let t42173 = 0.29792074959875355558e-1_f64 * t40395;
    let t42176 = t6710 * t6711 * t41839;
    let t42180 = t587 * t204 * t41878;
    let t42183 = t9267 * t2890 * t9278;
    let t42184 = 0.19171462976960374838e1_f64 * t42183;
    let t42185 = -t42144 - 0.51123901271894332901e0_f64 * t42146 - t42151 + t42154 + t42157 - t42159 - t42161 - 0.12423108009070322895e3_f64 * t42163 + 0.55213813373645879536e2_f64 * t42166 - t42168 - 0.38342925953920749676e0_f64 * t40374 - t42170 + 0.51123901271894332901e0_f64 * t40380 + t42172 + t42173 + 0.38342925953920749676e0_f64 * t40397 - 0.23005755572352449806e2_f64 * t42176 - 0.76685851907841499352e0_f64 * t40400 - 0.18404604457881959845e2_f64 * t42180 + t42184;
    t42185
}
