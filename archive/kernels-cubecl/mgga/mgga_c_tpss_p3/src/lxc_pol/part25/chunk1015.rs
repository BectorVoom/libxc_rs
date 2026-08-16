//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1015/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1015<F: Float>(t14052: F, t14003: F, t14029: F, t14034: F, t14036: F, t14040: F, t14046: F, t14050: F, t198: F, t2439: F, t3548: F, t3552: F, t3610: F, t4706: F, t740: F, t7929: F, t7932: F, t7936: F, t8000: F, t8019: F, t8023: F, t8024: F, t8029: F, t8030: F, t8040: F) -> (F, F) {
    let t14053 = F::cast_from(0.18311447306006545054e-3_f64) * t14052;
    let t14054 = F::cast_from(3.0_f64) * t14029 * t198 * t740 + F::cast_from(6.0_f64) * t198 * t4706 * t8030 + F::cast_from(6.0_f64) * t2439 * t3548 * t3610 + F::cast_from(6.0_f64) * t14040 * t3552 + F::cast_from(12.0_f64) * t14046 * t3552 - t14003 + t14034 + t14036 + t14050 - t14053 + t7929 - t7932 - t7936 + t8000 - t8019 + t8023 + t8024 - t8029 - t8040;
    (t14053, t14054)
}
