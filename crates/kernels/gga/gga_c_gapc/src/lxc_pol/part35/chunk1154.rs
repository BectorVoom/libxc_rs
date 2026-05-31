//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 1154/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk1154<F: Float>(t2964: F, t9370: F, t3179: F, t1616: F, t1615: F, t3655: F, t1617: F, t11302: F, t19844: F, t5974: F, t1743: F, t33148: F) -> (F, F, F, F, F) {
    let t34303 = F::cast_from(2.0_f64) * t2964 * t9370;
    let t34306 = t3179 * t3179;
    let t34308 = F::cast_from(4.0_f64) * t1616 * t34306;
    let t34311 = t3655 * t1615;
    let t34313 = F::cast_from(2.0_f64) * t34311 * t1617;
    let t34315 = t19844 * t11302 * t5974;
    let t34317 = t1743 * t33148;
    (t34303, t34308, t34313, t34315, t34317)
}
