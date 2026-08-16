//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta646 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2059;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2060;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta646<F: Float>(t1409: F, t1937: F, t6722: F, t14501: F, t23419: F, t1015: F, t23472: F, t25678: F, t7554: F, t82632: F, t225: F, t25820: F, t23384: F, t25827: F, t25436: F, t23328: F, t23394: F, t1054: F, t4693: F, t13783: F, t1926: F, t221: F, t25432: F, t25806: F, t6680: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t88692, t88704, t88723, t88731, t88744) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2059::<F>(t1409, t1937, t6722, t14501, t23419, t1015, t23472, t25678, t7554, t82632, t225, t25820);
        let (t88753, t88758, t88772, t88804, t88810, t88812, t88845) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2060::<F>(t23384, t25827, t25436, t23328, t23394, t1054, t4693, t13783, t1926, t221, t25432, t25806, t6680);
    (t88692, t88704, t88723, t88731, t88744, t88753, t88758, t88772, t88804, t88810, t88812, t88845)
}
