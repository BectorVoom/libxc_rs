//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1017/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1017<F: Float>(t10610: F, t12739: F, t12383: F, t3472: F, t3275: F, t1149: F, t2995: F, t12056: F, t3262: F, t3574: F, t3465: F, t8601: F) -> (F, F, F, F, F, F) {
    let t12740 = t10610 * t12739;
    let t12741 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t12740;
    let t12742 = t3472 * t12383;
    let t12743 = t3275 * t12742;
    let t12744 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t12743;
    let t12745 = t2995 * t1149;
    let t12747 = t3262 * t12056 * t3574;
    let t12748 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t12747;
    let t12751 = t3275 * t3465 * t8601;
    (t12741, t12742, t12744, t12745, t12748, t12751)
}
