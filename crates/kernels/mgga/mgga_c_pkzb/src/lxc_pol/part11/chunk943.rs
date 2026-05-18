//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 943/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk943<F: Float>(t10242: F, t10245: F, t10253: F, t10258: F, t10263: F, t10267: F, t10272: F, t10276: F, t10278: F, t2380: F, t3225: F, t3235: F, t3238: F, t6468: F, t8319: F, t8386: F, t8389: F, t8394: F, t8398: F, t8408: F, t8458: F, t8469: F, t8472: F) -> F {
    let t10280 = -t8386 - F::new(0.47637797908966374413e-4) * t6468 + t8389 - t8394 + F::new(0.28582678745379824648e-3) * t10242 - F::new(0.42874018118069736972e-3) * t2380 * t10245 + F::new(0.45732285992607719436e-2) * t8319 * t3225 + F::new(0.19055119163586549765e-3) * t8398 + F::new(0.12862205435420921092e-2) * t3235 * t10253 - F::new(0.13719685797782315831e-1) * t10258 * t3238 - F::new(0.51448821741683684368e-2) * t3235 * t10263 + F::new(0.25724410870841842184e-2) * t3235 * t10267 + F::new(0.85748036236139473947e-3) * t10272 - F::new(0.28582678745379824648e-3) * t10276 + F::new(0.30488190661738479624e-2) * t10278 + t8408 - t8458 + t8469 - t8472;
    t10280
}
