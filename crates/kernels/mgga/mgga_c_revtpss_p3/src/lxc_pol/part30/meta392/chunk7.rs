//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1472/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1472<F: Float>(t1892: F, t4086: F, t786: F, t4104: F, t2470: F, t5740: F, t4101: F, t1432: F, t5763: F, t1385: F, t5710: F, t10105: F, t10109: F, t10114: F, t10117: F, t10120: F, t10126: F, t10129: F, t10137: F, t10143: F, t13921: F, t1399: F, t1437: F, t3924: F, t4118: F, t5659: F, t5767: F, t820: F) -> F {
    let t14238 = t4086 * t1892;
    let t14239 = t786 * t14238;
    let t14241 = F::cast_from(0.19514881078765566038e-1_f64) * t14239 * t4104;
    let t14242 = t5740 * t2470;
    let t14243 = t4101 * t14242;
    let t14252 = t1432 * t5763 * t2470;
    let t14255 = t1385 * t5710;
    let t14266 = -t14241 + F::cast_from(0.13009920719177044025e-1_f64) * t14243 + F::cast_from(0.9757440539382783019e-2_f64) * t10105 + F::cast_from(0.23131639038696784278e-2_f64) * t10109 + t10114 - F::cast_from(0.13170898365871023197e1_f64) * t820 * t4118 * t5659 - t10117 - F::cast_from(0.9757440539382783019e-2_f64) * t10120 - t10126 - t10129 - F::cast_from(0.13009920719177044025e-1_f64) * t14252 + F::cast_from(0.2601984143835408805e-1_f64) * t10137 - F::cast_from(0.13170898365871023197e1_f64) * t820 * t14255 * t1399 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t5767 * t3924 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t1437 * t13921 - F::cast_from(0.23131639038696784278e-2_f64) * t10143;
    t14266
}
