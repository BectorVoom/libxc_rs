//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1402/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1402<F: Float>(t132: F, t1019: F, t10853: F, t11182: F, t1388: F, t1445: F, t2449: F, t2624: F, t29538: F, t30228: F, t30235: F, t30273: F, t30297: F, t30304: F, t30330: F, t30348: F, t30366: F, t340: F, t3455: F, t3609: F, t394: F, t4224: F, t4348: F, t8955: F, t9310: F, t932: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> F {
    let t133 = t132 <= zeta_threshold;
    let t134 = rho1 <= dens_threshold || t133;
    let t30373 = piecewise3::<f64>(t134, F::new(0.0), t29538 * t394 / F::new(2.0) + t10853 * t1019 + t4224 * t2624 / F::new(2.0) + t8955 * t1445 + F::new(2.0) * t3455 * t3609 + t1388 * t9310 + t2449 * t4348 / F::new(2.0) + t932 * t11182 + t340 * (t30228 + t30235 + t30273 + t30297 + t30304 + t30330 + t30348 + t30366) / F::new(2.0));
    t30373
}
