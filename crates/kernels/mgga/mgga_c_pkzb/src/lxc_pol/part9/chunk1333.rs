//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1333/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1333<F: Float>(t24: F, t22149: F, t1263: F, t1265: F, t22356: F, t22386: F, t22910: F, t23547: F, t23551: F, t23554: F, t23561: F, t23567: F, t2467: F, t2471: F, t3289: F, t3293: F, t422: F, t423: F, t6606: F, t6613: F, t8577: F, t8587: F, t960: F, t962: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> F {
    let t90 = t24 <= zeta_threshold;
    let t332 = rho1 <= dens_threshold || t90;
    let t23586 = piecewise3::<f64>(t90, F::new(0.0), -t22149);
    let t23590 = piecewise3::<f64>(t332, F::new(0.0), (t22356 + t22386 + t23547 + t23551 + t23554 + t23561 + t23567 + t22910) * t423 / F::new(2.0) + F::new(3.0) / F::new(2.0) * t8577 * t962 + F::new(3.0) / F::new(2.0) * t3289 * t2471 + t1263 * t6613 / F::new(2.0) + t6606 * t1265 / F::new(2.0) + F::new(3.0) / F::new(2.0) * t2467 * t3293 + F::new(3.0) / F::new(2.0) * t960 * t8587 + t422 * t23586 / F::new(2.0));
    t23590
}
