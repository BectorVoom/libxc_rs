//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 846/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk846<F: Float>(t1377: F, t2091: F, t3886: F, t794: F, t8611: F, t6897: F, t31153: F, t31160: F, t31177: F, t22674: F, t8621: F, t2085: F, t214: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t31549 = t1377 * t2091;
    let t31558 = t3886 * t2091;
    let t31569 = t794 * t8611;
    let t31570 = t6897 * t31569;
    let t31571 = F::cast_from(0.41123351671205660912e-2_f64) * t31570;
    let t31576 = F::cast_from(0.11304371706359309439e-1_f64) * t31153;
    let t31578 = F::cast_from(0.26915170729426927235e-3_f64) * t31160;
    let t31582 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t31177;
    let t31594 = t22674 * t8621;
    let t31595 = t6897 * t31594;
    let t31596 = F::cast_from(0.41123351671205660912e-2_f64) * t31595;
    let t31611 = t214 * t2085;
    (t31549, t31558, t31569, t31571, t31576, t31578, t31582, t31594, t31596, t31611)
}
