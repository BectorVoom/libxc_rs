//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1959/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1959<F: Float>(t1364: F, t28905: F, t786: F, t102113: F, t102117: F, t102120: F, t102122: F, t102129: F, t102131: F, t102133: F, t102135: F, t102139: F, t26241: F, t26246: F, t27837: F, t8095: F, t94610: F, t96206: F) -> F {
    let t102143 = F::cast_from(0.19514881078765566038e-1_f64) * t786 * t28905 * t1364;
    let t102148 = t102113 + t102117 + F::cast_from(0.4818682326780666368e-3_f64) * t102120 - F::cast_from(0.13009920719177044025e-1_f64) * t102122 + t96206 + F::cast_from(0.8673628188205199462e0_f64) * t94610 * t8095 - t102129 + F::cast_from(0.24093411633903331839e-3_f64) * t102131 + F::cast_from(0.17135234354032049604e-2_f64) * t102133 - F::cast_from(0.22849835011101738147e-2_f64) * t102135 - F::cast_from(0.65049603595885220126e-3_f64) * t102139 + t102143 + F::cast_from(0.4336814094102599731e0_f64) * t27837 * t26246 + F::cast_from(0.8673628188205199462e0_f64) * t27837 * t26241;
    t102148
}
