//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 888/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk888<F: Float>(t2958: F, t5397: F, t2580: F, t169: F, t299: F, t8720: F, t706: F, t738: F, t8833: F, t1030: F, t1897: F, t1908: F, t1935: F, t270: F, t2964: F, t681: F, t7168: F, t7175: F, t7179: F, t7182: F, t7184: F, t7188: F, t7190: F, t7194: F, t7207: F, t7212: F, t7215: F) -> (F, F, F) {
    let t8970 = t2958 * t5397;
    let t8971 = t2580 * t8970;
    let t8979 = t8720 * t169 * t299;
    let t8980 = t706 * t8979;
    let t8983 = t738 * t8833;
    let t8988 = F::new(0.17090058289204942853e-2) * t7168 - F::new(0.64087718584518535698e-3) * t7175 - F::new(0.64087718584518535698e-3) * t7179 + F::new(0.64087718584518535698e-3) * t7182 - F::new(0.17090058289204942853e-2) * t7184 - F::new(0.1281754371690370714e-2) * t7188 + F::new(0.1281754371690370714e-2) * t7190 + F::new(0.17090058289204942853e-2) * t7194 - F::new(0.17090058289204942853e-2) * t7207 + F::new(0.64087718584518535698e-3) * t7212 + F::new(0.1281754371690370714e-2) * t7215 - F::new(0.30762104920568897134e-1) * t1897 * t8971 - F::new(0.76905262301422242837e-2) * t1935 * t1030 - F::new(0.15381052460284448567e-1) * t681 * t2964 + F::new(0.76905262301422242837e-2) * t270 * t8980 - F::new(0.76905262301422242837e-2) * t270 * t8983 - F::new(0.34180116578409885707e-2) * t1908 * t1030;
    (t8970, t8979, t8988)
}
