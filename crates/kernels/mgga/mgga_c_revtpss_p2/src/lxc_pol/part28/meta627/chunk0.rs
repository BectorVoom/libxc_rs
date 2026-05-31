//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2247/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2247<F: Float>(t27799: F, t98779: F, t1711: F, t2394: F, t2430: F, t27375: F, t94245: F, t61155: F, t2832: F, t1113: F, t4537: F, t1940: F, t1963: F, t2403: F, t25206: F, t25440: F, t25767: F, t27364: F, t27382: F, t27777: F, t27802: F, t27810: F, t27817: F, t4541: F, t51780: F, t7087: F, t7091: F, t7783: F, t7863: F, t99542: F) -> F {
    let t101065 = t27799 * t98779;
    let t101070 = t1711 * t2394;
    let t101074 = t1711 * t2430;
    let t101083 = t94245 * t27375;
    let t101086 = t27799 * t61155;
    let t101093 = t1711 * t2832;
    let t101099 = t1113 * t4537;
    let t101105 = t27382 * t101065 + F::cast_from(3.0_f64) * t2403 * t7087 * t27810 - t99542 + F::cast_from(3.0_f64) * t4541 * t1963 * t101070 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t1963 * t101074 + F::cast_from(3.0_f64) * t2403 * t7087 * t27777 - t1940 * t25440 * t27802 - F::cast_from(3.0_f64) * t25206 * t101083 + F::cast_from(3.0_f64) * t25206 * t101086 - t1940 * t25440 * t27817 + F::cast_from(3.0_f64) * t51780 * t7863 - t1940 * t7091 * t101093 / F::cast_from(2.0_f64) + t1940 * t27364 * t1113 - t1940 * t7091 * t101099 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t7783 * t25767;
    t101105
}
