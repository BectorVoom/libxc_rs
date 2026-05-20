//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1206/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1206<F: Float>(t102610: F, t102629: F, t102636: F, t108282: F, t109706: F, t109715: F, t109858: F, t114640: F, t114666: F, t1904: F, t25930: F, t26304: F, t27837: F, t30071: F, t30267: F, t8095: F, t8104: F, t94823: F, t96549: F, t96564: F, t96584: F, t96591: F) -> F {
    let t115258 = -F::cast_from(0.32927245914677557992e-1_f64) * t109715 - F::cast_from(0.72280234901709995519e-3_f64) * t102610 - F::cast_from(0.13010442282307799193e1_f64) * t30071 * t8104 - F::cast_from(0.51405703062096148814e-2_f64) * t102629 - F::cast_from(0.72280234901709995519e-3_f64) * t102636 + F::cast_from(0.13010442282307799193e1_f64) * t27837 * t30267 + F::cast_from(0.78062653693846795158e1_f64) * t94823 * t26304 * t114666 - F::cast_from(0.26020884564615598386e1_f64) * t25930 * t26304 * t114640 + t96549 - F::cast_from(0.19756347548806534796e1_f64) * t109706 * t1904 - t96564 + F::cast_from(0.26020884564615598386e1_f64) * t108282 * t8095 - t96584 - F::cast_from(0.29272321618148349057e-1_f64) * t109858 + t96591;
    t115258
}
