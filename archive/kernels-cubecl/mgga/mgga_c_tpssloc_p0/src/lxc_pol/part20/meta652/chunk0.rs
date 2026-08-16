//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2400/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2400<F: Float>(t41662: F, t41675: F, t41678: F, t41682: F, t41684: F, t41863: F, t41865: F, t41870: F, t41872: F, t41874: F, t41876: F, t48982: F) -> F {
    let t49167 = F::cast_from(0.10064166666666666667e0_f64) * t41662 + F::cast_from(0.80513333333333333335e0_f64) * t41675 - F::cast_from(0.40256666666666666668e0_f64) * t41678 + F::cast_from(0.60385000000000000002e0_f64) * t41682 + F::cast_from(0.93932222222222222223e0_f64) * t41684 + F::cast_from(0.16504875e0_f64) * t48982 + F::cast_from(0.73586666666666666668e0_f64) * t41863 - F::cast_from(0.11038e0_f64) * t41865 - F::cast_from(0.27595e0_f64) * t41870 - F::cast_from(0.91983333333333333335e-1_f64) * t41872 + F::cast_from(0.5519e-1_f64) * t41874 + F::cast_from(0.24528888888888888889e-1_f64) * t41876;
    t49167
}
