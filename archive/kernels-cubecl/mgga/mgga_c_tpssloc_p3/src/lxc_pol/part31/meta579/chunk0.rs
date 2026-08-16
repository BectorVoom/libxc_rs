//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1816/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1816<F: Float>(t26197: F, t80670: F, t1834: F, t213: F, t225: F, t80711: F, t22724: F, t26474: F, t22751: F, t26194: F, t1887: F, t80830: F) -> (F, F, F, F, F, F) {
    let t90551 = t80670 * t26197;
    let t90566 = t213 * t1834 * t225;
    let t90581 = F::cast_from(0.52089578783527170489e-1_f64) * t80711;
    let t90582 = t22724 * t26474;
    let t90584 = t22751 * t26194;
    let t90591 = t80830 * t1887;
    (t90551, t90566, t90581, t90582, t90584, t90591)
}
