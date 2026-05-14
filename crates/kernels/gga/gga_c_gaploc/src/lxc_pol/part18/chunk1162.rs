//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1162/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1162<F: Float>(t33473: F, t24751: F, t2676: F, t24945: F, t11116: F, t22263: F, t32357: F, t4820: F, t7513: F, t32436: F, t10678: F, t11095: F, t1402: F, t1628: F, t2033: F, t33453: F, t33455: F, t33458: F, t33460: F, t33462: F, t33465: F, t33469: F, t3495: F, t4598: F, t813: F) -> (F,) {
    let t33474 = 0.51123901271894332902e0 * t33473;
    let t33476 = 0.23833659967900284446e0 * t24751 * t2676;
    let t33478 = 0.23833659967900284446e0 * t24945 * t2676;
    let t33480 = 0.15889106645266856297e0 * t22263 * t11116;
    let t33483 = 0.15889106645266856297e0 * t7513 * t4820 * t32357;
    let t33486 = 0.15889106645266856297e0 * t7513 * t4820 * t32436;
    let t33487 = -0.61348681526273199482e1 * t813 * t1628 * t11095 - 0.1022478025437886658e1 * t813 * t4598 * t3495 - t33453 + t33455 - t33458 + t33460 - t33462 - t33465 + t33469 - 0.92686455430723328401e-1 * t2033 * t1402 * t10678 - t33474 + t33476 + t33478 - t33480 - t33483 - t33486;
    (t33487,)
}
