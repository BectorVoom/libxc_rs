//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1250/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1250<F: Float>(t322: F, t41940: F, t41971: F, t42003: F, t42035: F, t42067: F, t42098: F, t42131: F, t12029: F, t37271: F, t12094: F, t37282: F, t12215: F, t40549: F) -> (F, F, F, F) {
    let t323 = t322 <= F::cast_from(0.0_f64);
    let t331 = t322 <= F::cast_from(0.25e1_f64);
    let t42133 = piecewise5::<F>(t323, t41940, t331, t41971 + t42003 + t42035 + t42067, t42098 + t42131);
    let t42136 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t37271 * t12029;
    let t42138 = F::cast_from(15.0_f64) / F::cast_from(8.0_f64) * t37282 * t12094;
    let t42140 = F::cast_from(3.0_f64) * t40549 * t12215;
    (t42133, t42136, t42138, t42140)
}
