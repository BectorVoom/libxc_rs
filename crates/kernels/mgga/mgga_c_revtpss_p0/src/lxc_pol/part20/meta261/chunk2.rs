//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1106/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1106<F: Float>(t11291: F, t11293: F, t11296: F, t11303: F, t11382: F, t11390: F, t11521: F, t11525: F, t11530: F, t11533: F, t11547: F, t11548: F, t11551: F, t11554: F, t11557: F, t11572: F, t11585: F, t2945: F, t2968: F, t2987: F, t2989: F, t3012: F, t311: F) -> F {
    let t11588 = -F::cast_from(0.35089341735807877242e1_f64) * t2987 * t11521 + F::cast_from(0.51947577317044391277e2_f64) * t3012 * t11525 + t11530 - t11533 + t11547 - t11291 - t11293 - t11296 + t11303 - t11382 - t11390 - F::new(6.0) * t11548 * t2945 + F::new(6.0) * t2968 * t11551 - F::cast_from(0.35089341735807877242e1_f64) * t11554 * t2989 + F::cast_from(0.35089341735807877242e1_f64) * t3012 * t11557 - F::cast_from(0.19751673498613801407e-1_f64) * t11572 - F::new(0.310907e-1) * t11585 * t311;
    t11588
}
