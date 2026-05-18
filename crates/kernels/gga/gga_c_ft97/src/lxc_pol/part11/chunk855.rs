//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 855/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk855<F: Float>(t1624: F, t1631: F, t1696: F, t22590: F, t372: F, t374: F, t37435: F, t37443: F, t37445: F, t37455: F, t37458: F, t37459: F, t37464: F, t37473: F, t37477: F, t37483: F, t37484: F, t37488: F, t37495: F, t37499: F, t37504: F, t37506: F, t37509: F, t37518: F, t37519: F, t388: F, t401: F, t428: F, t7838: F, t7839: F, t7845: F, t7861: F, t7889: F, t7978: F, t8000: F, t8001: F, t8002: F, t8009: F, t8165: F, t8169: F, t8173: F) -> F {
    let t37522 = -F::new(0.39217632015950386692e-4) * t37435 * t8001 * t8002 * t1696 * t401 + F::new(0.19608816007975193346e-4) * t37443 * t8001 * t37445 + F::new(0.82704389902445944777e-3) * t7845 * t8165 * t7839 - F::new(0.38465647900339007384e-5) * t37455 * t7861 + F::new(0.9009618584720619741e0) * t22590 * t37458 * t37459 * t401 - F::new(0.1422571355482203117e0) * t22590 * t37458 * t37464 * t401 + F::new(0.14225713554822031171e0) * t7889 * t37458 * t37464 * t428 + F::new(0.2845142710964406234e0) * t388 * t37473 * t37477 + F::new(0.57000134242798356259e-7) * t37483 * t37484 - F::new(0.32032606786708831383e-6) * t37488 * t7861 + F::new(0.46509801892875584e-1) * t1624 * t374 * t7978 * t428 + F::new(0.93019603785751168e-2) * t372 * t1631 * t37495 + F::new(0.12418916805050955786e-3) * t8000 * t8001 * t37499 - F::new(0.11019649358382880326e-4) * t37504 * t37506 - F::new(0.1744777815077289385e-3) * t8009 * t37509 - F::new(0.16540877980489188955e-2) * t7838 * t8169 * t7839 - F::new(0.16540877980489188956e-2) * t7838 * t8173 * t7839 + F::new(0.11019649358382880326e-3) * t37518 * t37519;
    t37522
}
