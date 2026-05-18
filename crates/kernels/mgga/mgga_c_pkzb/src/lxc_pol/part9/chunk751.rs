//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 751/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk751<F: Float>(t1634: F, t1734: F, t179: F, t5135: F, t66: F, t168: F, t167: F, t180: F, t1706: F, t1733: F, t2592: F, t2645: F, t5222: F, t5225: F, t5227: F, t5232: F, t5236: F, t5241: F, t5244: F, t5247: F, t5252: F, t5258: F, t5261: F, t5265: F, t5267: F, t5270: F, t5275: F, t5279: F, t580: F) -> (F, F, F, F) {
    let t5281 = t179 * t1734 * t1634;
    let t5285 = F::new(1.0) / t66 / t5135;
    let t5286 = t168 * t5285;
    let t5289 = F::new(0.37792653007779990369e-1) * t167 * t5286 * t180;
    let t5290 = -F::new(7.0) / F::new(16.0) * t5222 - t5225 * t5227 / F::new(4.0) + F::new(0.25724410870841842183e-2) * t1733 * t5232 + F::new(0.25724410870841842183e-2) * t1733 * t5236 - F::new(0.64311027177104605458e-3) * t2645 * t5241 - F::new(0.51448821741683684367e-2) * t5244 * t5247 + F::new(0.12862205435420921092e-2) * t2592 * t5252 - F::new(0.24009450146119052704e-1) * t5258 + F::new(3.0) / F::new(16.0) * t1706 * t5261 - F::new(35.0) / F::new(72.0) * t5265 + F::new(7.0) / F::new(48.0) * t5267 - t580 * t5270 / F::new(48.0) + F::new(0.25724410870841842183e-2) * t1733 * t5275 - F::new(0.12862205435420921092e-1) * t5279 * t5281 - t5289;
    (t5281, t5286, t5289, t5290)
}
