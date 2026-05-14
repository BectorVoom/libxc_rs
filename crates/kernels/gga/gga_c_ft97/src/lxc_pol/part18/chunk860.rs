//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 860/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk860<F: Float>(t23294: F, t379: F, t1909: F, t1647: F, t5717: F, t447: F, t5750: F, t22950: F, t83: F, t1334: F, t8232: F, t1882: F, t5745: F, t1307: F, t1922: F, t452: F) -> (F, F, F, F, F, F, F, F, F) {
    let t23295 = t23294 * t379;
    let t23296 = t1909 * t23295;
    let t23299 = t5717 * t1647;
    let t23300 = t1909 * t23299;
    let t23304 = t447 * t5750 * t379;
    let t23307 = t83 * t22950;
    let t23311 = 4.0 / 27.0 * t8232 * t1334;
    let t23312 = t1882 * t5745;
    let t23315 = t452 * t1922 * t1307;
    (t23295, t23296, t23299, t23300, t23304, t23307, t23311, t23312, t23315)
}
