//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1268/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1268<F: Float>(t10610: F, t3472: F, t42432: F, t11465: F, t12567: F, t11325: F, t12395: F, t3262: F, t12945: F, t37282: F, t12215: F, t42945: F) -> (F, F, F, F, F) {
    let t44926 = F::cast_from(15.0_f64) / F::cast_from(8.0_f64) * t10610 * t3472 * t42432;
    let t44928 = F::cast_from(5.0_f64) / F::cast_from(16.0_f64) * t12567 * t11465;
    let t44931 = F::cast_from(15.0_f64) / F::cast_from(8.0_f64) * t3262 * t11325 * t12395;
    let t44933 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t37282 * t12945;
    let t44935 = F::cast_from(3.0_f64) * t42945 * t12215;
    (t44926, t44928, t44931, t44933, t44935)
}
