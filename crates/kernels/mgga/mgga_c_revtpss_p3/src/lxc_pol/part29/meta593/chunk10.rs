//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1987/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1987<F: Float>(t102615: F, t102617: F, t102622: F, t102629: F, t102634: F, t102636: F, t14230: F, t14269: F, t25909: F, t27868: F, t28008: F, t28899: F, t28912: F, t4078: F, t7511: F, t7532: F, t8104: F, t96516: F, t96527: F, t96542: F, t96546: F, t97855: F) -> F {
    let t102642 = -F::cast_from(0.23131639038696784278e-2_f64) * t96516 - t102615 + t102617 - F::cast_from(0.4336814094102599731e0_f64) * t25909 * t8104 + F::cast_from(0.13170898365871023197e1_f64) * t28899 * t4078 - F::cast_from(0.17347256376410398924e1_f64) * t27868 * t102622 * t14230 + F::cast_from(0.14456046980341999104e-1_f64) * t96527 - F::cast_from(0.65854491829355115987e0_f64) * t7511 * t14269 - F::cast_from(0.17135234354032049604e-2_f64) * t102629 - F::cast_from(0.8673628188205199462e0_f64) * t28008 * t7532 + t102634 - F::cast_from(0.24093411633903331839e-3_f64) * t102636 - F::cast_from(0.17347256376410398924e1_f64) * t97855 * t28912 - F::cast_from(0.14456046980341999104e-1_f64) * t96542 + F::cast_from(0.96373646535613327358e-3_f64) * t96546;
    t102642
}
