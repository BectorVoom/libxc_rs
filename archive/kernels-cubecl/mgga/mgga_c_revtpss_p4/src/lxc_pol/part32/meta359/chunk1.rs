//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1303/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1303<F: Float>(t221: F, t2675: F, t4343: F, t2674: F, t243: F, t4423: F, t231: F, t2662: F, t2661: F, t10722: F, t1565: F, t4352: F, t4366: F) -> (F, F, F, F, F, F) {
    let t14857 = t2675 * t221 * t4343;
    let t14859 = F::cast_from(0.10164000561857065645e-3_f64) * t2674 * t14857;
    let t14860 = t243 * t4423;
    let t14861 = t14860 * t231;
    let t14862 = t2662 * t14861;
    let t14864 = F::cast_from(0.14291339372689912324e-4_f64) * t2661 * t14862;
    let t14866 = t10722 * t1565;
    let t14868 = t4352 * t4366;
    (t14857, t14859, t14861, t14864, t14866, t14868)
}
