//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1129/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1129(t2380: f64, t2417: f64, t278: f64, t808: f64, t9600: f64, t2360: f64, t274: f64, t2349: f64, t10313: f64, t10321: f64, t10326: f64, t10328: f64, t10334: f64, t10339: f64, t10355: f64, t2014: f64, t231: f64, t2394: f64, t2704: f64, t2705: f64, t2710: f64, t39942: f64, t39976: f64, t41622: f64, t43204: f64, t43208: f64, t43210: f64, t683: f64, t688: f64, t703: f64, t801: f64, t8948: f64, t8959: f64, t8963: f64, t9525: f64, t9609: f64) -> f64 {
    let t43651 = t2417 * t2380 * t278;
    let t43656 = t808 * t9600;
    let t43691 = t274 * t2360;
    let t43692 = t43691 * t2349;
    let t43702 = -0.438942848081465325e0_f64 * t43208 * t274 - 0.35115427846517226e0_f64 * t43210 * t274 - 0.5498505610292168117e-2_f64 * t10355 * t43651 - 0.30699166922921429856e0_f64 * t10339 * t43651 + 0.1279131955121726244e0_f64 * t2710 * t43656 - 0.15095674251318553494e0_f64 * t9609 * t43651 + 0.55909904634513161088e-1_f64 * t2394 * t43656 - 0.44273842265453930305e-2_f64 * t8963 * t703 * t2417 * t688 * t10328 + 0.48229216329983294636e-3_f64 * t8963 * t703 * t9525 * t801 * t10328 + 0.22136921132726965153e-3_f64 * t39942 * t10313 + 0.19923229019454268637e-2_f64 * t8948 * t683 * t2704 * t2417 - 0.10625722143708943273e-1_f64 * t2014 * t231 * t9600 * t688 * t274 - 0.90429780618718677442e-4_f64 * t8948 * t683 * t41622 * t801 * t274 - 0.44273842265453930305e-2_f64 * t8959 * t10321 + 0.17709536906181572122e-2_f64 * t8963 * t10326 * t43692 + 0.48229216329983294636e-3_f64 * t8959 * t10334 + 0.23410285231011484e0_f64 * t43204 * t274 + 0.59031789687271907073e-3_f64 * t39976 * t2705;
    t43702
}
