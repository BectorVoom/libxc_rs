//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1059/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1059<F: Float>(t11767: F, t11770: F, t11779: F, t11782: F, t11785: F, t11787: F, t11756: F, t11762: F, t11773: F, t11776: F, t12193: F, t12194: F, t12195: F, t12196: F, t12197: F, t12198: F, t12199: F, t12200: F, t12203: F) -> F {
    let t12204 = F::cast_from(0.34752370105806885418e-3_f64) * t11767;
    let t12205 = F::cast_from(0.1422820120100248667e-7_f64) * t11770;
    let t12208 = F::cast_from(0.16908181191593721013e-5_f64) * t11779;
    let t12209 = F::cast_from(0.24760339692676868218e-5_f64) * t11782;
    let t12210 = F::cast_from(0.10551281119038438161e-7_f64) * t11785;
    let t12211 = F::cast_from(0.10551281119038438161e-7_f64) * t11787;
    let t12212 = t12193 + t12194 - t12195 + t12196 + t12197 - t12198 - t12199 + t12200 - F::cast_from(0.252977417353824213e-7_f64) * t11756 + F::cast_from(0.12228868272569444446e-4_f64) * t11762 - t12203 - t12204 + t12205 + F::cast_from(0.12650553385416666668e-5_f64) * t11773 + F::cast_from(0.12650553385416666668e-5_f64) * t11776 + t12208 + t12209 + t12210 + t12211;
    t12212
}
