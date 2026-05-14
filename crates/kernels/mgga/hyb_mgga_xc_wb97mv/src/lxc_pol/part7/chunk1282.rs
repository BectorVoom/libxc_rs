//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1282/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1282<F: Float>(t10980: F, t260: F, t10947: F, t11203: F, t22427: F, t2318: F, t2326: F, t2333: F, t26544: F, t26549: F, t30797: F, t31413: F, t3439: F, t3447: F, t3457: F, t4229: F, t4256: F, t4260: F, t4263: F, t7034: F, t855: F, t857: F, t8934: F, t8937: F, t9002: F, t9158: F, t9230: F) -> (F,) {
    let t31441 = t260 * t10980;
    let t31470 = 0.11696447245269292414e1 * t7034 * t4256 - t30797 - 0.11696447245269292414e1 * t31441 * t857 + 0.23392894490538584828e1 * t3447 * t8937 - 0.5848223622634646207e0 * t7034 * t4260 - 0.23392894490538584828e1 * t8934 * t3457 - 0.11696447245269292414e1 * t3447 * t9002 - 0.35089341735807877242e1 * t855 * t4263 * t2318 - 0.6233709278045326953e3 * t855 * t10947 * t2326 - 0.69263436422725855036e2 * t2333 * t11203 + 0.12304822629859687989e5 * t855 * t22427 * t4229 * t9230 + 0.4155806185363551302e3 * t26544 * t3439 * t31413 - 0.41016075432865626631e4 * t26549 * t9158 * t31413;
    (t31470,)
}
