//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1160/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1160<F: Float>(t21643: F, t34363: F, t21801: F, t5395: F, t5743: F, t1743: F, t5722: F, t1030: F, t33311: F, t3714: F, t1036: F, t11316: F, t13483: F) -> (F, F, F, F, F, F, F) {
    let t34364 = t34363 * t21643;
    let t34366 = t5395 * t21801;
    let t34367 = t34366 * t5743;
    let t34370 = t1743 * t21801 * t5722;
    let t34372 = t1030 * t33311;
    let t34373 = t34372 * t3714;
    let t34378 = t11316 * t1036 * t13483;
    (t34364, t34366, t34367, t34370, t34372, t34373, t34378)
}
