//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2210/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2210<F: Float>(t100006: F, t100008: F, t100019: F, t100024: F, t100025: F, t100030: F, t12116: F, t12160: F, t15703: F, t16022: F, t16091: F, t16205: F, t27492: F, t27498: F, t3120: F, t3299: F, t4896: F, t4902: F, t7132: F, t93555: F, t93564: F) -> F {
    let t100035 = t100006 + F::cast_from(0.11433071498151929859e-2_f64) * t100008 * t16091 + F::cast_from(0.17149607247227894789e-2_f64) * t12116 * t27492 * t4896 - F::cast_from(0.85748036236139473944e-3_f64) * t12160 * t27492 * t4902 - F::cast_from(0.42874018118069736972e-3_f64) * t27498 * t16022 - F::cast_from(0.91464571985215438873e-2_f64) * t3299 * t100019 * t4896 + t100024 - F::cast_from(0.85748036236139473944e-3_f64) * t100025 * t3120 + F::cast_from(0.47637797908966374413e-3_f64) * t7132 * t16205 - F::cast_from(0.11433071498151929859e-2_f64) * t100030 * t15703 - F::cast_from(0.1270341277572436651e-3_f64) * t93555 + F::cast_from(0.28582678745379824648e-3_f64) * t93564;
    t100035
}
