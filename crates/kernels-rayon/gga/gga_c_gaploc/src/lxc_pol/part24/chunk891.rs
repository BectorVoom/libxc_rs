//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 891/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk891(t2958: f64, t5397: f64, t2580: f64, t169: f64, t299: f64, t8720: f64, t706: f64, t738: f64, t8833: f64, t1030: f64, t1897: f64, t1908: f64, t1935: f64, t270: f64, t2964: f64, t681: f64, t7168: f64, t7175: f64, t7179: f64, t7182: f64, t7184: f64, t7188: f64, t7190: f64, t7194: f64, t7207: f64, t7212: f64, t7215: f64) -> (f64, f64, f64) {
    let t8970 = t2958 * t5397;
    let t8971 = t2580 * t8970;
    let t8979 = t8720 * t169 * t299;
    let t8980 = t706 * t8979;
    let t8983 = t738 * t8833;
    let t8988 = 0.17090058289204942853e-2_f64 * t7168 - 0.64087718584518535698e-3_f64 * t7175 - 0.64087718584518535698e-3_f64 * t7179 + 0.64087718584518535698e-3_f64 * t7182 - 0.17090058289204942853e-2_f64 * t7184 - 0.1281754371690370714e-2_f64 * t7188 + 0.1281754371690370714e-2_f64 * t7190 + 0.17090058289204942853e-2_f64 * t7194 - 0.17090058289204942853e-2_f64 * t7207 + 0.64087718584518535698e-3_f64 * t7212 + 0.1281754371690370714e-2_f64 * t7215 - 0.30762104920568897134e-1_f64 * t1897 * t8971 - 0.76905262301422242837e-2_f64 * t1935 * t1030 - 0.15381052460284448567e-1_f64 * t681 * t2964 + 0.76905262301422242837e-2_f64 * t270 * t8980 - 0.76905262301422242837e-2_f64 * t270 * t8983 - 0.34180116578409885707e-2_f64 * t1908 * t1030;
    (t8970, t8979, t8988)
}
