//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1000/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1000<F: Float>(t10754: F, t750: F, t2133: F, t3657: F, t10552: F, t778: F, t10735: F, t10745: F, t10751: F, t1373: F, t1375: F, t222: F, t224: F, t2353: F, t2358: F, t2361: F, t3650: F, t3656: F, t3658: F, t3661: F, t776: F, t779: F) -> F {
    let t10755 = t10754 * t750;
    let t10758 = t3657 * t2133;
    let t10761 = t778 * t10552;
    let t10764 = -t10735 * t224 - F::cast_from(24.0_f64) * t10745 * t3658 + F::cast_from(60.0_f64) * t10751 * t3656 - F::cast_from(24.0_f64) * t10755 * t3656 - F::cast_from(12.0_f64) * t10758 * t3656 + F::cast_from(3.0_f64) * t10761 * t222 - F::cast_from(12.0_f64) * t1373 * t2358 + F::cast_from(3.0_f64) * t1373 * t2361 + F::cast_from(3.0_f64) * t1375 * t2353 + F::cast_from(6.0_f64) * t3650 * t779 + F::cast_from(6.0_f64) * t3661 * t776;
    t10764
}
