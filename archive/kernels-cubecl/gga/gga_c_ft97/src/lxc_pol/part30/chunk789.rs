//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 789/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk789<F: Float>(t19135: F, t28558: F, t28652: F, t28660: F, t31465: F, t33415: F, t33436: F, t33447: F, t33889: F, t33894: F, t33899: F, t33908: F, t33912: F, t33925: F, t33928: F, t33934: F, t33935: F, t33941: F, t33942: F, t33947: F, t33948: F) -> F {
    let t33951 = -F::cast_from(0.20527106943485609994e0_f64) * t19135 * t33889 + F::cast_from(0.18125821328051150223e0_f64) * t28652 * t33894 - F::cast_from(0.18125821328051150223e0_f64) * t28660 * t33899 + t33925 + F::cast_from(0.30209702213418583705e-1_f64) * t28558 * t33415 - F::cast_from(0.45306850413028723348e0_f64) * t33928 * t33908 + F::cast_from(0.22653425206514361674e0_f64) * t31465 * t33912 + F::cast_from(0.80027204934668021496e-1_f64) * t33934 * t33436 * t33935 - F::cast_from(0.12004080740200203224e0_f64) * t33941 * t33436 * t33942 + t33947 + F::cast_from(0.26675734978222673832e-1_f64) * t33948 * t33447;
    t33951
}
