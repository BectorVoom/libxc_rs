//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 786/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk786<F: Float>(t5325: F, t597: F, t1860: F, t1853: F, t625: F, t645: F, t182: F, t518: F, t190: F, t1696: F, t750: F, t1827: F, t732: F, t1842: F, t5274: F, t5278: F, t5282: F, t5283: F, t5288: F, t5295: F, t5298: F, t5302: F, t5303: F, t5307: F, t5321: F, t5323: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5326 = t597 * t5325;
    let t5327 = t1860 * t5326;
    let t5331 = 0.71233333333333333332e-1 * t625 * t1853 * t645;
    let t5332 = t518 * t182;
    let t5335 = 0.55403703703703703703e-1 * t625 * t5332 * t190;
    let t5336 = t1696 * t750;
    let t5338 = t732 * t1827;
    let t5340 = t732 * t1842;
    let t5342 = t5274 - t5278 + t5282 - 0.35089341735807877242e1 * t5283 - t5288 - t5295 + t5298 + t5302 + 0.51947577317044391277e2 * t5303 + t5307 + t5321 + 0.8103123984e0 * t5323 + 0.4051561992e0 * t5327 - t5331 + t5335 + 0.51947577317044391277e2 * t5336 - 0.70178683471615754484e1 * t5338 + 0.10389515463408878255e3 * t5340;
    (t5326, t5327, t5331, t5332, t5335, t5336, t5338, t5340, t5342)
}
