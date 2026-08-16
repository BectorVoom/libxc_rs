//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1037/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1037<F: Float>(t11853: F, t1214: F, t248: F, t11616: F, t68: F, t484: F, t10913: F, t4972: F, t4582: F, t1174: F, t11821: F, t11825: F, t11834: F, t11836: F, t11839: F, t11842: F, t11845: F, t11850: F, t1213: F, t1227: F, t1232: F, t3490: F, t3527: F, t3531: F, t3587: F, t488: F) -> F {
    let t11855 = t248 * t1214 * t11853;
    let t11858 = t11616 * t68;
    let t11859 = t11858 * t484;
    let t11862 = t4972 * t10913;
    let t11863 = t4582 * t11862;
    let t11866 = -t11821 / F::cast_from(4608.0_f64) + F::cast_from(5.0_f64) / F::cast_from(4608.0_f64) * t3490 * t3587 - t11825 * t1232 / F::cast_from(1536.0_f64) - t3490 * t3527 / F::cast_from(1536.0_f64) - t3490 * t3531 / F::cast_from(768.0_f64) + t11834 + t11836 / F::cast_from(432.0_f64) - t11839 / F::cast_from(288.0_f64) - t11842 / F::cast_from(144.0_f64) - t1174 * t11845 / F::cast_from(288.0_f64) - t1174 * t11850 / F::cast_from(48.0_f64) + t1213 * t11855 / F::cast_from(3072.0_f64) + t11859 * t488 / F::cast_from(3072.0_f64) - t1227 * t11863 / F::cast_from(768.0_f64);
    t11866
}
