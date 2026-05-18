//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 943/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk943<F: Float>(t276: F, t285: F, t2881: F, t918: F, t273: F, t2439: F, t931: F, t2915: F, t698: F, t11315: F, t916: F, t2880: F) -> (F, F, F, F, F, F) {
    let t11354 = F::new(1.0) / t276 / t285 / F::new(4.0);
    let t11355 = t2881 * t918;
    let t11356 = t11354 * t11355;
    let t11358 = F::new(1.0)/pow_3_2::<f64>(t273);
    let t11359 = t11358 * t11355;
    let t11366 = t2439 * t931;
    let t11368 = t698 * t2915;
    let t11370 = t916 * t11315;
    let t11372 = t2880 * t918;
    (t11356, t11359, t11366, t11368, t11370, t11372)
}
