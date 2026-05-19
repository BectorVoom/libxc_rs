//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1291/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1291<F: Float>(t3135: F, t6233: F, t1208: F, t1209: F, t18513: F, t18866: F, t18875: F, t18878: F, t22542: F, t22544: F, t2258: F, t2279: F, t2296: F, t2297: F, t2312: F, t2318: F, t3103: F, t3106: F, t3136: F, t3139: F, t6122: F, t6224: F, t6275: F, t6282: F, t6288: F, t6300: F, t6308: F, t6323: F, t8170: F, t8174: F, t8177: F, t8178: F, t8181: F) -> F {
    let t22662 = t3135 * t6233;
    let t22681 = -t22542 + F::cast_from(0.51947577317044391277e2_f64) * t6300 * t8174 + F::cast_from(0.30762056574649219973e4_f64) * t18875 * t8178 - F::cast_from(0.35089341735807877242e1_f64) * t2296 * t3136 * t2312 - F::cast_from(0.31168546390226634765e3_f64) * t6323 * t8170 * t2297 - F::cast_from(0.11696447245269292414e1_f64) * t2296 * t1209 * t6224 - F::cast_from(0.12304822629859687989e5_f64) * t18866 * t8177 * t6122 + F::cast_from(0.51947577317044391277e2_f64) * t2318 * t8170 * t2312 + F::cast_from(0.30762056574649219973e4_f64) * t6282 * t22662 * t2297 + F::cast_from(0.17315859105681463759e2_f64) * t2318 * t3139 * t6224 + F::cast_from(0.91082604192152556044e5_f64) * t18878 * t1208 * t18513 * t6122 + F::new(18.0) * t6308 * t8181 + F::new(18.0) * t2279 * t3103 * t2258 + F::cast_from(0.11579025239058625248e4_f64) * t6288 * t3106 * t6275 - t22544;
    t22681
}
