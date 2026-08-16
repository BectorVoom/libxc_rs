//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1012/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1012<F: Float>(t75820: F, t75823: F, t75825: F, t2228: F, t2350: F, t903: F, t15467: F, t4601: F, t1550: F, t699: F, t8704: F, t75859: F) -> (F, F, F, F, F, F, F) {
    let t78311 = F::cast_from(0.5959043985061697516e-4_f64) * t75820;
    let t78312 = F::cast_from(0.2553875993597870364e-4_f64) * t75823;
    let t78313 = F::cast_from(0.2553875993597870364e-4_f64) * t75825;
    let t78321 = t903 * t2228 * t2350;
    let t78322 = F::cast_from(0.44903406381989282115e-1_f64) * t78321;
    let t78323 = t4601 * t15467;
    let t78324 = F::cast_from(0.44903406381989282115e-1_f64) * t78323;
    let t78326 = t1550 * t699 * t8704;
    let t78327 = F::cast_from(0.2993560425465952141e-1_f64) * t78326;
    let t78339 = F::cast_from(0.44903406381989282115e-1_f64) * t75859;
    (t78311, t78312, t78313, t78322, t78324, t78327, t78339)
}
