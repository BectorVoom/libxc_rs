//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2315/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2315<F: Float>(t90983: F, t1336: F, t1352: F, t16033: F, t16055: F, t1825: F, t22879: F, t26404: F, t26442: F, t26453: F, t26458: F, t3773: F, t3777: F, t3851: F, t5234: F, t5344: F, t7747: F, t81199: F, t90942: F, t90946: F, t90952: F, t90957: F, t90962: F, t90964: F, t90968: F, t90971: F, t90980: F) -> F {
    let t90984 = F::cast_from(0.82246703342411321824e-2_f64) * t90983;
    let t90985 = -F::cast_from(2.0_f64) * t5344 * t90942 * t1352 - F::cast_from(2.0_f64) * t5344 * t90946 * t1352 + F::cast_from(4.0_f64) * t16055 * t26453 - F::cast_from(2.0_f64) * t1336 * t90952 * t1352 + t90957 - t1336 * t26458 * t3851 - t90962 - t90964 + t3773 * t7747 + F::cast_from(0.16449340668482264365e-1_f64) * t90968 + t90971 - t5234 * t22879 - t1336 * t81199 * t1825 - F::cast_from(2.0_f64) * t3777 * t26442 - F::cast_from(2.0_f64) * t16033 * t26404 + F::cast_from(0.82246703342411321824e-2_f64) * t90980 + t90984;
    t90985
}
