//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1307/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1307<F: Float>(t676: F, t9387: F, t2629: F, t9372: F, t2434: F, t2516: F, t8779: F, t9645: F, t252: F, t685: F, t788: F, t2452: F, t588: F) -> (F, F, F, F, F, F, F, F, F) {
    let t39532 = t676 * t9387;
    let t39534 = F::cast_from(0.21687162600603479684e-1_f64) * t2629 * t39532;
    let t39535 = t676 * t9372;
    let t39537 = F::cast_from(0.38025319932552508021e2_f64) * t2629 * t39535;
    let t39538 = t2434 * t2516;
    let t39540 = F::cast_from(0.43374325201206959368e-1_f64) * t2629 * t39538;
    let t39545 = t8779 * t9645;
    let t39549 = F::cast_from(0.65457331274007190912e-5_f64) * t39545 * t252 * t788 * t685;
    let t39552 = t588 * t2452;
    (t39532, t39534, t39535, t39537, t39538, t39540, t39545, t39549, t39552)
}
