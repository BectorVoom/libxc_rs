//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 574/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk574<F: Float>(t1424: F, t1459: F, t1463: F, t1470: F, t1480: F, t1488: F, t1511: F, t1526: F, t1529: F, t1533: F, t2821: F, t2824: F, t3020: F, t3036: F, t3038: F) -> F {
    let t3173 = -t1424 - t1511 + F::cast_from(0.571528e-1_f64) * t2821 + t1459 - t1526 - F::cast_from(0.1350520664e0_f64) * t2824 + t3020 + t1470 - t1480 - t1488 - t3038 - t3036 - t1529 + t1463 - t1533;
    t3173
}
