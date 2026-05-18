//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 830/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk830<F: Float>(t44294: F, t6508: F, t1358: F, t6507: F, t2339: F, t35918: F, t42581: F, t10231: F, t1365: F, t42529: F, t44258: F, t44262: F, t44263: F, t44264: F, t44267: F, t44269: F, t44278: F, t44281: F, t44284: F, t44288: F, t44292: F, t44293: F, t7888: F) -> (F, F) {
    let t44295 = t6508 * t44294;
    let t44298 = F::new(0.63233348079280332442e-2) * t1358 * t6507 * t44295;
    let t44301 = F::new(0.22131671827748116354e-1) * t1358 * t35918 * t2339;
    let t44302 = F::new(0.18970004423784099733e-1) * t42581;
    let t44303 = -t44258 + F::new(0.47425011059460249332e-2) * t42529 + t44262 + t44263 - t44264 - t44267 + F::new(0.31616674039640166221e-2) * t1358 * t1365 * t44269 + F::new(0.18970004423784099732e-1) * t1358 * t7888 * t10231 - t44278 + t44281 - t44284 + t44288 - t44292 - t44293 - t44298 + t44301 + t44302;
    (t44295, t44303)
}
