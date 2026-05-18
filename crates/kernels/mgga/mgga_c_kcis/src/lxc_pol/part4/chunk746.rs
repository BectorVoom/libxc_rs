//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 746/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk746<F: Float>(t1307: F, t1610: F, t4440: F, t1444: F, t617: F, t2642: F, t1600: F, t1601: F, t2645: F, t1606: F, t616: F, t494: F) -> (F, F, F, F, F, F, F, F) {
    let t4441 = t1307 * t1610;
    let t4442 = t4440 * t4441;
    let t4445 = t617 * t1444;
    let t4446 = t4445 * t2642;
    let t4447 = t1600 * t4446;
    let t4450 = t1601 * t2645;
    let t4451 = t1600 * t4450;
    let t4455 = F::new(1.0) / t1606 / t616;
    let t4456 = t494 * t4455;
    (t4441, t4442, t4446, t4447, t4450, t4451, t4455, t4456)
}
